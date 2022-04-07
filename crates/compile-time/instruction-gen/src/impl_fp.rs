use vm::{MembAccessFp, RoutineFp};

use crate::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(crate) fn routine_fp(&self, routine: EntityRoutePtr) -> Option<RoutineFp> {
        match self.db.entity_source(routine).unwrap() {
            EntitySource::Builtin(builtin_entity_data) => match builtin_entity_data.decl {
                StaticEntityDecl::Func(ref func_decl) => Some(func_decl.compiled),
                StaticEntityDecl::Ty {
                    raw_ty_kind,
                    visualizer,
                } => todo!(),
                StaticEntityDecl::TyTemplate => todo!(),
                StaticEntityDecl::Module => todo!(),
                StaticEntityDecl::Trait { .. } => todo!(),
            },
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::WithinModule { .. } => self
                .db
                .linkage_table()
                .entity_routine(self.db.entity_uid(routine)),
            EntitySource::Module { file } => todo!(),
            EntitySource::Contextual { main, ident } => todo!(),
        }
    }

    pub(crate) fn memb_access_fp(
        &self,
        this_ty: EntityRoutePtr,
        memb_ident: CustomIdentifier,
    ) -> Option<MembAccessFp> {
        self.db
            .linkage_table()
            .memb_access(self.db.entity_uid(this_ty), memb_ident)
    }

    pub(crate) fn memb_routine_fp(
        &self,
        this_ty: EntityRoutePtr,
        memb_ident: CustomIdentifier,
    ) -> Option<RoutineFp> {
        self.db
            .linkage_table()
            .memb_routine(self.db.entity_uid(this_ty), memb_ident)
    }
}