use std::io;
use std::io::Write;

struct Game{
    board : [[char; 3] ;3],
    round : u8
}

impl Game {
    fn PrintGame(&mut self) {
        let mut row_i = 1;
        println!("\n\n-GAME-ROUND-{}", self.round);
        println!("-------------");
        println!("    1 2 3 ");
        println!("   ------ ");
        for row in self.board{
            print!(" {}|",row_i);
            row_i += 1;

            for e in row{
                print!(" {e}");
            }
            print!("\n");
        }
        println!("");
    }
    
    fn PutPieceCheck(&mut self, x:u8, y:u8) -> bool{
        if x>=3 {
            return false;
        }
        if y>=3 {
            return false;
        }
        if self.board[x as usize][y as usize] != '-'{
            return false;
        }
        return true;
    }

    fn PutPiece(&mut self, x:u8, y:u8) -> bool{
        let mut piece = '1';
        if self.round % 2 ==0{
            piece = '0';
        }
        if self.PutPieceCheck(x,y) {
            self.board[x as usize][y as usize] = piece;
        } else {
            return false;
        }
        self.round += 1;
        true
    }

    fn CheckWinner(&mut self) -> bool{
        for i in 0..2{
            if (self.board[i][0] == self.board[i][1]) &&
            (self.board[i][0] == self.board[i][2]) && 
            (self.board[i][0] != '-'){
                return true;
            }
        }
        for i in 0..2{
            if (self.board[0][i] == self.board[1][i]) &&
            (self.board[0][i] == self.board[2][i]) && 
            (self.board[0][i] != '-'){
                return true;
            }
        }
        if (self.board[0][0] == self.board[1][1]) &&
            (self.board[1][1] == self.board[2][2]) && 
            (self.board[0][0] != '-'){
            return true;
        }
        if (self.board[0][2] == self.board[1][1]) &&
            (self.board[1][1] == self.board[2][0]) && 
            (self.board[0][2] != '-'){
            return true;
        }
        return false;
    }

    fn GetUserInput(&mut self) -> [u8;2]{
        loop {
            if self.round % 2 == 0{
                print!("Player 1 >");
            } else {
                print!("Player 2 >");
            }
            std::io::stdout().flush().unwrap();
            let mut uin = String::new();
            io::stdin()
                .read_line(&mut uin)
                .expect("read fail");
    
            let info = uin.trim();
            if info.len() != 2{
                println!("Please type with right format ex: '12'");
                continue;
            }
            let info_chars:Vec<char> = info.chars().collect();
            let x = info_chars[0].to_digit(10).unwrap() as u8;
            let y = info_chars[1].to_digit(10).unwrap() as u8;
            return [x-1 ,y-1];
        }
    }
}


fn main() {
    println!("Welcome to our Rust tic_tac_toe");
    let mut bd = Game{
        board : [['-';3];3],
        round : 0
    };

    loop {
        bd.PrintGame();
        let location = bd.GetUserInput();
        if bd.PutPiece(location[0], location[1]) == false {
            println!("Please Put Piece on the right place!");
            continue;
        }
        if bd.CheckWinner(){
            bd.PrintGame();
            println!("You Win the Game! Good Job!");
            break;
        }
    }
}
