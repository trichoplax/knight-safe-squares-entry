fn main() {
    let attacking_squares:[(i8, i8); 8] = [(-1, -2), (1, -2), (-2, -1), (2, -1), (-2, 1), (2, 1), (-1, 2), (1, 2)];
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

        let mut output = 64;
        for y in 0..8i8 {
            for x in 0..8i8 {
                if exploded_board[y as usize][x as usize] == 'N' {
                    output -= 1
                } else {
                    for (a, b) in attacking_squares {
                        let (attacking_x, attacking_y):(i8, i8) = (x + a, y + b);
                        if attacking_x >= 0 && attacking_x <= 7 && attacking_y >= 0 && attacking_y <= 7 && exploded_board[attacking_y as usize][attacking_x as usize] == 'N' {
                            output -= 1;
                            break;
                        }
                    }
                }
            }
        }

        let bitwise_output = |input_integer: u64| {
            // Moves in direction (2, -1)
            let mask = u64::from_str_radix("\
                00000000\
                11111100\
                11111100\
                11111100\
                11111100\
                11111100\
                11111100\
                11111100\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved1 = pieces_to_move << 6;

            // Moves in direction (1, -2)
            let mask = u64::from_str_radix("\
                00000000\
                00000000\
                11111110\
                11111110\
                11111110\
                11111110\
                11111110\
                11111110\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved2 = pieces_to_move << 15;

            // Moves in direction (-1, -2)
            let mask = u64::from_str_radix("\
                00000000\
                00000000\
                01111111\
                01111111\
                01111111\
                01111111\
                01111111\
                01111111\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved3 = pieces_to_move << 17;

            // Moves in direction (-2, -1)
            let mask = u64::from_str_radix("\
                00000000\
                00111111\
                00111111\
                00111111\
                00111111\
                00111111\
                00111111\
                00111111\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved4 = pieces_to_move << 10;

            // Moves in direction (-2, 1)
            let mask = u64::from_str_radix("\
                00111111\
                00111111\
                00111111\
                00111111\
                00111111\
                00111111\
                00111111\
                00000000\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved5 = pieces_to_move >> 6;

            // Moves in direction (-1, 2)
            let mask = u64::from_str_radix("\
                01111111\
                01111111\
                01111111\
                01111111\
                01111111\
                01111111\
                00000000\
                00000000\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved6 = pieces_to_move >> 15;

            // Moves in direction (1, 2)
            let mask = u64::from_str_radix("\
                11111110\
                11111110\
                11111110\
                11111110\
                11111110\
                11111110\
                00000000\
                00000000\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved7 = pieces_to_move >> 17;

            // Moves in direction (2, 1)
            let mask = u64::from_str_radix("\
                11111100\
                11111100\
                11111100\
                11111100\
                11111100\
                11111100\
                11111100\
                00000000\
            ", 2).unwrap();
            let pieces_to_move = input_integer & mask;
            let moved8 = pieces_to_move >> 10;

            (input_integer | moved1 | moved2 | moved3 | moved4 | moved5 | moved6 | moved7 | moved8).count_zeros()
        };

        let bitwise = bitwise_output(input_integer);
        let inverse_input_integer = u64::MAX - input_integer;

        assert!(bitwise == output, "True output: {}\nGolfed output: {}\n\n", output, bitwise);
        println!("{input_integer} : {inverse_input_integer} : {bitwise}");
    }
}
