use crate::*;

pub(crate) fn collect_diagnostics(
    this: &dyn DiagnosticQuery,
    module: scope::PackageOrModule,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if let Ok(table) = this.subscope_table(module.scope_id()) {
        diagnostics.extend(table.error_iter().map(|e| e.into()));
        diagnostics.extend(
            this.subscope_ids(module.scope_id())
                .iter()
                .map(|subscope_id| match this.scope_kind(*subscope_id) {
                    scope::ScopeKind::Module => todo!(),
                    _ => collect_module_def_diagnostics(this, *subscope_id),
                })
                .flatten(),
        );
    }
    diagnostics
}

fn collect_module_def_diagnostics(
    _this: &dyn DiagnosticQuery,
    _scope: scope::ScopeId,
) -> Vec<Diagnostic> {
    msg_once!("todo collect module def diagnostics!");
    Vec::new()
}