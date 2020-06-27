use pleco;
use rand::seq::SliceRandom;

pub fn shuffled_move_list(it: pleco::MoveList) -> Vec<pleco::BitMove> {
    let mut rng = rand::thread_rng();
    let mut y: Vec<pleco::BitMove> = it.iter().map(|x| *x).collect();
    y.shuffle(&mut rng);
    return y;
}
