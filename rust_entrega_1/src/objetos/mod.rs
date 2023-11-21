use ambiente::Ambiente;
use esfera::Esfera;
use luz::Luz;
use plano::Plano;

pub(crate) mod ambiente;
pub(crate) mod esfera;
pub(crate) mod luz;
pub(crate) mod plano;
pub enum Objetos {
    Esfera(Esfera),
    Plano(Plano),
}

pub enum Fontes {
    Ambiente(Ambiente),
    Luz(Luz),
}
