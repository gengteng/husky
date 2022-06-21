use super::*;

#[derive(Debug, Default)]
pub struct AttentionContext {
    pub attention: Rc<Signal<Attention>>,
    last_attention: RefCell<Attention>,
    attention_locked_store: Signal<bool>,
}
impl AttentionContext {
    pub(super) fn init(&self, attention: Attention) {
        self.attention_locked_store.set(true);
        self.attention.set(attention);
    }

    pub(super) fn opt_input_id(&self) -> Option<usize> {
        return self.attention.get().opt_sample_id();
    }

    pub(super) fn did_lock_attention(&mut self, attention: Attention) {
        self.attention.set(attention);
        self.attention_locked_store.set(true);
    }

    pub(super) fn attention(&self) -> Attention {
        return self.attention.cget();
    }

    pub(super) fn toggled_attention_kind(&self) -> Attention {
        let last_last_attention = self.last_attention.replace(self.attention.cget());

        if std::mem::discriminant(&last_last_attention)
            != std::mem::discriminant(&self.last_attention.borrow(file!(), line!()))
        {
            last_last_attention
        } else {
            match *self.last_attention.borrow(file!(), line!()) {
                Attention::Specific { .. } => Attention::default(),
                Attention::Generic { .. } => Attention::Specific {
                    sample_id: ask_for_input_id(),
                },
            }
        }
    }
}

fn ask_for_input_id() -> usize {
    let window = web_sys::window().unwrap();
    let mut last_error: Option<String> = None;
    loop {
        let answer = match last_error {
            Some(error) => window.prompt_with_message(&format!("{:?}\ninput id = ", error)),
            None => window.prompt_with_message("input id = "),
        };
        match answer {
            Ok(Some(input_id_str)) => match input_id_str.parse::<usize>() {
                Ok(input_id) => break input_id,
                Err(e) => {
                    last_error = Some(format!("expect a valid number, but get {:?} instead", e))
                }
            },
            Ok(None) => last_error = Some(format!("expect a valid number, but get nothing")),
            Err(ref e) => last_error = Some(js_sys::JSON::stringify(e).unwrap().into()),
        }
    }
}