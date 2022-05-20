use super::*;

#[derive(PartialEq, Eq, Clone)]
pub struct SubrouteEntry {
    pub ident: Option<CustomIdentifier>,
    pub kind: EntityKind,
    pub source: EntitySource,
}

impl std::fmt::Debug for SubrouteEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ident: {:?}, kind: {:?}}}, source: {:?}",
            self.ident, self.kind, self.source
        ))
    }
}

impl SubrouteEntry {
    pub fn from_token_group(
        db: &dyn EntityRouteSalsaQueryGroup,
        file: FilePtr,
        parent_entity_kind: EntityKind,
        token_group_index: usize,
        token_group: &[Token],
    ) -> EntitySyntaxResult<Option<SubrouteEntry>> {
        if token_group[0].kind == TokenKind::Keyword(Keyword::Use.into()) {
            return Ok(None);
        }
        if token_group.len() < 2 {
            match token_group[0].kind {
                TokenKind::Identifier(Identifier::Custom(ident)) => {
                    return Ok(Some(SubrouteEntry {
                        ident: Some(ident),
                        kind: EntityKind::EnumLiteral,
                        source: EntitySource::from_file(file, token_group_index),
                    }))
                }
                _ => {
                    return Err(defn_error!(
                        "invalid single token for entity defn head",
                        token_group[0].text_range()
                    ))
                }
            }
        }
        if token_group.len() == 2 {
            return match token_group[0].kind {
                TokenKind::Keyword(Keyword::Main) => Ok(Some(SubrouteEntry {
                    ident: None,
                    kind: EntityKind::Main,
                    source: EntitySource::from_file(file, token_group_index),
                })),
                TokenKind::Keyword(Keyword::Mod) => {
                    SubrouteEntry::submodule(db, file, token_group_index, token_group)
                }
                _ => Ok(None),
            };
        }
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                if let TokenKind::Identifier(ident) = token_group[1].kind {
                    if let Some(kind) = tell_entity_kind(keyword, &token_group[2]) {
                        return match ident {
                            Identifier::Builtin(_) => Err(defn_error!(
                                "builtin identifiers are reserved",
                                token_group[1].text_range()
                            )),
                            Identifier::Custom(ident) => Ok(Some(SubrouteEntry {
                                ident: Some(ident),
                                kind,
                                source: EntitySource::from_file(file, token_group_index),
                            })),
                            Identifier::Contextual(_) => Err(defn_error!(
                                "contextual identifiers are reserved",
                                token_group[1].text_range()
                            )),
                        };
                    }
                }
                Err(defn_error!(
                    "second token should be identifier",
                    token_group[1].text_range()
                ))
            }
            TokenKind::Decorator(Decorator::Static) => match token_group[1].kind {
                TokenKind::Keyword(Keyword::Routine(routine_keyword)) => {
                    match token_group[2].kind {
                        TokenKind::Identifier(Identifier::Custom(ident)) => {
                            Ok(Some(SubrouteEntry {
                                ident: Some(ident),
                                kind: EntityKind::Routine,
                                source: EntitySource::from_file(file, token_group_index),
                            }))
                        }
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            },
            _ => Ok(None),
        }
    }

    pub fn submodule(
        db: &dyn EntityRouteSalsaQueryGroup,
        file: FilePtr,
        token_group_index: usize,
        token_group: &[Token],
    ) -> EntitySyntaxResult<Option<SubrouteEntry>> {
        let ident = match token_group[1].kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => ident,
            _ => todo!(),
        };
        if let Some(submodule_file) = db.get_submodule_file(&file, ident) {
            Ok(Some(SubrouteEntry {
                ident: Some(ident),
                kind: EntityKind::Module,
                source: EntitySource::Module {
                    file: submodule_file,
                },
            }))
        } else {
            Err(defn_error!(
                format!("file for submodule doesn't exist"),
                token_group.text_range()
            ))
        }
    }
}