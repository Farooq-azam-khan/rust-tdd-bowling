
fn get_score(game_state: &[u32]) -> u32 {
    let mut score: u32 = 0; 
    let mut cursor = 0;
    while cursor < game_state.len()-1 {
        let is_strike = cursor < game_state.len() -1 && game_state[cursor] == 10; 
        let is_spare = game_state[cursor] + game_state[cursor+1] == 10;
        if is_strike {
            score += 10 + game_state[cursor+1] + game_state[cursor+2];
            cursor += 1;
        } else if  is_spare {
            score += 10 + game_state[cursor+2];
            cursor += 2;
        } else {
            score += game_state[cursor] + game_state[cursor+1];
            cursor += 2;
        }
    }
    score 
}

#[test]
fn test_game_state_init() {
    let game_state: [u32; 21] = [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 0); 
}

#[test]
fn test_score_with_1_pin() {
    let game_state: [u32; 21] = [1,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 1); 
}


#[test]
fn test_score_with_1_frame_complete() {
    let game_state: [u32; 21] = [3,3, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 6); 
}


#[test]
fn test_score_with_1_spare_only() {
    let game_state: [u32; 21] = [5,5, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 10); 
}

#[test]
fn test_score_with_1_spare_followed_by_3() {
    let game_state: [u32; 21] = [5,5, 3,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 16); 
}

#[test]
fn test_score_with_1_strike_only() {
    let game_state: [u32; 21] = [10,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 10); 
}


#[test]
fn test_score_with_1_strike_followed_by_3_2() {
    let game_state: [u32; 20] = [10, 3,2, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0];
    assert_eq!(get_score(&game_state), 20); 
}

#[test]
fn test_perfect_game() {
    let game_state: [u32; 21] = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0,0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(get_score(&game_state), 300); 
}