use file::FilePtr;
use static_defn::EntityStaticDefn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitySource {
    StaticModuleItem(&'static EntityStaticDefn),
    StaticTypeMember,
    StaticTypeAsTraitMember,
    WithinBuiltinModule,
    WithinModule {
        file: FilePtr,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file: FilePtr,
    },
    Input {
        main: FilePtr,
    },
}

impl EntitySource {
    pub fn from_file(file_id: FilePtr, token_group_index: usize) -> EntitySource {
        EntitySource::WithinModule {
            file: file_id,
            token_group_index: token_group_index,
        }
    }
}

impl From<&'static EntityStaticDefn> for EntitySource {
    fn from(data: &'static EntityStaticDefn) -> Self {
        Self::StaticModuleItem(data)
    }
}