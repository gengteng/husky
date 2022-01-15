use crate::*;

#[salsa::query_group(DiagnosticQueryStorage)]
pub trait DiagnosticQuery: scope::ScopeQueryGroup {
    fn diagnostic_reserve(&self, module: scope::PackageOrModule) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(
    this: &dyn DiagnosticQuery,
    module: scope::PackageOrModule,
) -> Arc<DiagnosticReserve> {
    Arc::new(DiagnosticReserve::new(collect_diagnostics(this, module)))
}