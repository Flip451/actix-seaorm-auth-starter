use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::TracerProvider;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    // 1. エクスポーターの構築
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://jaeger:4317")
        .build()
        .expect("Failed to create OTLP exporter");

    // 2. サービス名などのリソース情報を定義
    let resource = Resource::new(vec![
        KeyValue::new("service.name", "myapp-service"),
        KeyValue::new("service.version", "1.0.0"),
    ]);

    // 3. トレーサープロバイダーの構築（リソースを指定）
    let provider = TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_resource(resource) // リソースをセット
        .build();

    // 3. トレーサーを取得
    let tracer = provider.tracer("myapp-service");

    // 4. グローバルな TracerProvider として登録（後続の tracing レイヤーで使用）
    opentelemetry::global::set_tracer_provider(provider);

    // 5. tracing-subscriber レイヤーの構築
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer()) // コンソール出力
        .with(tracing_opentelemetry::layer().with_tracer(tracer)) // OTel 出力
        .init();
}

pub fn shutdown() {
    opentelemetry::global::shutdown_tracer_provider();
}
