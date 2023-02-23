enum CellState {
    Dead,
    Alive,
}

fn calc_next_board_state(board: &Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {

}

#[cfg(test)]
mod test {
    use super::calc_next_board_state;
    use super::CellState::*;

    #[test]
    fn test_1() {
        #[rustfmt::skip]
        let initial_state = vec![
            vec![Dead,  Alive, Dead], 
            vec![Dead,  Dead,  Alive], 
            vec![Alive, Alive, Alive], 
            vec![Dead,  Dead,  Dead]
        ];

        #[rustfmt::skip]
        let final_state = vec![
            vec![Dead,  Dead,  Dead], 
            vec![Alive, Dead,  Alive], 
            vec![Dead,  Alive, Alive], 
            vec![Dead,  Alive, Dead]
        ];

        let calc_next_state = calc_next_board_state(&initial_state);

        assert_eq!(final_state, calc_next_state);
    }
}
