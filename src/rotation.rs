use crate::game::Rot;

type Kick = (i32, i32);
type RotationPair = (Rot, Rot);

pub const KICK_MAP: [(RotationPair, [Kick; 5]); 8] = [
    ((Rot::R0, Rot::R1), [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]),
    ((Rot::R1, Rot::R0), [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]),
    ((Rot::R1, Rot::R2), [(0, 0), (1, 0), (-1, 1), (0, -2), (-1, -2)]),
    ((Rot::R2, Rot::R1), [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]),
    ((Rot::R2, Rot::R3), [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]),
    ((Rot::R3, Rot::R2), [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]),
    ((Rot::R3, Rot::R0), [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]),
    ((Rot::R0, Rot::R3), [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]),
];

pub const KICK_MAP_I: [(RotationPair, [Kick; 5]); 8] = [
    ((Rot::R0, Rot::R1), [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]),
    ((Rot::R1, Rot::R0), [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]),
    ((Rot::R1, Rot::R2), [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]),
    ((Rot::R2, Rot::R1), [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]),
    ((Rot::R2, Rot::R3), [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]),
    ((Rot::R3, Rot::R2), [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]),
    ((Rot::R3, Rot::R0), [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]),
    ((Rot::R0, Rot::R3), [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]),
];

pub fn get_kicks(
    from: Rot,
    to: Rot,
    is_i_piece: bool,
) -> &'static [(i32, i32); 5] {
    let table = if is_i_piece {
        &KICK_MAP_I
    } else {
        &KICK_MAP
    };

    table
        .iter()
        .find(|(rot, _)| *rot == (from, to))
        .map(|(_, kicks)| kicks)
        .expect("Invalid rotation pair")
}