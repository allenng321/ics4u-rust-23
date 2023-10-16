fn main() {
	let board = ['X', 'O', 'X', 'O', 'X', '-', '-', 'X', 'O'];

	display(board)
}

fn display(b: [char; 9]) {
    for i in 0..b.len() {
        if i % 3 == 0{
            print!("\n{}  ", b[i]);
        } else {
            print!("{}  ", b[i]);
        }
    }
    println!();
}

fn is_win(b: [char;9]) -> bool {
	if (b[0] == b[1] && b[1] == b[2]) || (b[3] == b[4] && b[4] == b[5]) || (b[6] == b[7] && b[7] ==  b[8]) 
    || (b[0] == b[3] && b[3] == b[6]) || (b[1] == b[4] && b[4] == b[7]) || (b[2] == b[5] && b[5] == b[8])
    || (b[0] == b[4] && b[4] == b[8]) || (b[2] == b[4] && b[4] == b[6]) {return true;}
    else {return false;}
}

fn is_tie(b: [char;9]) -> bool {
	for _i in 0..b.len() {
        if b[_i] == '-' {return false;}
    }
    return true;
}