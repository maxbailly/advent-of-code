use super::WireRef;

/* ---------- */

#[derive(Default, Debug)]
pub struct Wires(Vec<WireRef>);

impl Wires {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entry(&mut self, id: &'static str) -> WireRef {
        self.0.iter()
            .find(|wire| wire.id() == id)
            .cloned()
            .unwrap_or_else(|| {
                let new = WireRef::new(id);

                self.push(new.clone());
                new
            })
    }

    #[inline]
    pub fn push(&mut self, new: WireRef) {
        if !self.0.iter().any(|wire| wire.id() == new.id()) {
            self.0.push(new)
        }
    }

    #[inline]
    pub fn reset(&self) {
        self.0.iter().for_each(|wire| wire.reset())
    }

    #[inline]
    pub fn wire(&self, id: &'static str) -> Option<&WireRef> {
        self.0.iter().find(|wire| wire.id() == id)
    }
}
