fn main() {

    let input_cases = [
        [
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********"
        ],
        [
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN"
        ],
        [
            "N*N*N*N*",
            "*N*N*N*N",
            "N*N*N*N*",
            "*N*N*N*N",
            "N*N*N*N*",
            "*N*N*N*N",
            "N*N*N*N*",
            "*N*N*N*N"
        ],
        [
            "N***N***",
            "*N***N**",
            "**N***N*",
            "***N***N",
            "N***N***",
            "*N***N**",
            "**N***N*",
            "***N***N"
        ],
        [
            "N*******",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********"
        ],
        [
            "******N*",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********"
        ],
        [
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "*****N**"
        ],
        [
            "********",
            "********",
            "********",
            "********",
            "********",
            "********",
            "*N******",
            "********"
        ],
        [
            "********",
            "**N*****",
            "********",
            "********",
            "********",
            "********",
            "********",
            "********"
        ],
        [
            "********",
            "********",
            "**N*****",
            "********",
            "********",
            "********",
            "********",
            "********"
        ],
        [
            "********",
            "********",
            "****N***",
            "********",
            "***N****",
            "********",
            "********",
            "********"
        ],
        [
            "********",
            "********",
            "***N****",
            "*****N**",
            "**N*****",
            "****N***",
            "********",
            "********"
        ],
        [
            "NNNNNNNN",
            "NNN*N*NN",
            "NN*NNN*N",
            "NNNN*NNN",
            "NN*NNN*N",
            "NNN*N*NN",
            "NNNNNNNN",
            "NNNNNNNN"
        ],
        [
            "NNNNNNNN",
            "NNN*N*NN",
            "N***NN*N",
            "*NNN*NNN",
            "NN*NNN*N",
            "*NN***NN",
            "N*N*NNNN",
            "NNNNNNNN"
        ],
        [
            "*NNNNNNN",
            "NN*NNNNN",
            "N*NNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN"
        ],
        [
            "NNNNNN*N",
            "NNNN*NNN",
            "NNNNN*N*",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN"
        ],
        [
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNN*N*N",
            "NNN*NNN*",
            "NNNNN*NN"
        ],
        [
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "*N*NNNNN",
            "NNN*NNNN",
            "N*NNNNNN",
            "NNN*NNNN"
        ],
        [
            "*NNN*NNN",
            "NN*NNNNN",
            "*NNN*NNN",
            "N*N*NNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN"
        ],
        [
            "N*N*NNNN",
            "*NNN*NNN",
            "NN*NNNNN",
            "*NNN*NNN",
            "N*N*NNNN",
            "NNNNNNNN",
            "NNNNNNNN",
            "NNNNNNNN"
        ],
        [
            "********",
            "********",
            "**N*****",
            "********",
            "********",
            "********",
            "******N*",
            "N*******"
        ],
    ];

    // Alternative version taking input as an unsigned 64 bit integer
    // input u64 : output as before
    for case in input_cases {
        let mut input_integer = 0;
        let mut bit_value: u64 = 1 << 63;

        for row in case {
            let squares = row.chars();
            for square in squares {
                if square == 'N' {
                    input_integer += bit_value;
                }
                bit_value >>= 1;
            }
        }

        let exploded_board: Vec<_> = case
            .iter()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect();

        let vector_based_output = |exploded_board: Vec<Vec<char>>| {
            let k=|a,b|exploded_board[a as usize][b as usize]=='N';
            let attacking_squares = [(-1,-2),(1,-2),(-2,-1),(2,-1),(-2,1),(2,1),(-1,2),(1,2)];
            let mut output = 64;
            for y in 0..8{
                for x in 0..8{
                    if k(y,x){
                        output -= 1
                    } else {
                        for (a, b) in attacking_squares {
                            if x+a >= 0 && x+a < 8 && y+b >= 0 && y+b < 8 && k(y+b,x+a) {
                                output -= 1;
                                break;
                            }
                        }
                    }
                }
            }

            output
        };

        let bitwise_output = |i:u64|(i|(i&71209857637481724)<<6|(i&280371153272574)<<15|(i&140185576636287)<<17|(i&17802464409370431)<<10|(i&4557430888798830336)>>6|(i&9187201950435704832)>>15|(i&18374403900871409664)>>17|(i&18229723555195321344)>>10).count_zeros();

        let bitwise = bitwise_output(input_integer);
        let output = vector_based_output(exploded_board);
        let inverse_input_integer = u64::MAX - input_integer;

        assert!(bitwise == output, "True output: {}\nGolfed output: {}\n\n", output, bitwise);
        println!("{input_integer} : {inverse_input_integer} : {bitwise}");
    }
}
