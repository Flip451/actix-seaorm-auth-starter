#[macro_export]
macro_rules! impl_responder_for {
    ($target: ty) => {
        impl actix_web::Responder for $target {
            type Body = actix_web::body::BoxBody;

            fn respond_to(
                self,
                _req: &actix_web::HttpRequest,
            ) -> actix_web::HttpResponse<Self::Body> {
                actix_web::HttpResponse::Ok().json(self)
            }
        }
    };
}
