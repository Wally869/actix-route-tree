use actix_web::{
    Error, FromRequest, Handler, HttpResponse, Responder, Route,
    http::Method,
    web::{self, ServiceConfig},
};

pub async fn get_agent_capabilities(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn get_tests() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub struct RouteBuilder {   
    // (path, route)
    routes: Vec<(String, Route)>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        RouteBuilder {
            routes: Vec::new(),
        }
    }

    pub fn get<F, Args>(mut self, path: &str, handler: F) -> Self 
    where 
        F: Handler<Args> + 'static,
        Args: FromRequest + 'static,
        F::Output: Responder + 'static,
    {
        let route = web::get().to(handler);
        self.routes.push((path.to_string(), route));
        self
    }

    pub fn post<F, Args>(mut self, path: &str, handler: F) -> Self 
    where 
        F: Handler<Args> + 'static,
        Args: FromRequest + 'static,
        F::Output: Responder + 'static,
    {
        let route = web::post().to(handler);
        self.routes.push((path.to_string(), route));
        self
    }

    pub fn sort(&mut self) {
        self.routes.sort_by(|a, b| {
            let static_count_a = a.0.split('/').filter(|s| !s.starts_with('{')).count();
            let static_count_b = b.0.split('/').filter(|s| !s.starts_with('{')).count();
            
            // More static segments = higher priority (comes first)
            static_count_b.cmp(&static_count_a)
        });
    }

    pub fn sort_and_flush(mut self, cfg: &mut ServiceConfig) {
        // Sort by specificity (static segments count)
        self.routes.sort_by(|a, b| {
            let static_count_a = a.0.split('/').filter(|s| !s.starts_with('{')).count();
            let static_count_b = b.0.split('/').filter(|s| !s.starts_with('{')).count();
            
            // More static segments = higher priority (comes first)
            static_count_b.cmp(&static_count_a)
        });

        // Apply routes in sorted order
        for (path, route) in self.routes {
            cfg.route(&path, route);
        }
    }
}

fn configure_routes(cfg: &mut ServiceConfig) {
    RouteBuilder::new()
        .get("/test/{id}", get_agent_capabilities)    // Will be ordered after static routes
        .get("/test/search", get_tests)               // Will be ordered first (static)
        .get("/test", get_tests)                      // Will be ordered second (static, shorter)
        .sort_and_flush(cfg);                         // Magic happens here!
}


