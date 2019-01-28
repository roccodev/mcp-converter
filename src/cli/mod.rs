pub fn parse_args(args: &Vec<String>) {

    if args.len() < 2 {
        eprintln!("Arguments must be more than 1, supplied {}.", args.len() - 1);
        return;
    }

    let mode = &args[1];
    let chars: Vec<char> = mode.chars().collect();

    if chars.len() != 3 {
        eprintln!("Invalid format: please use [f/c/p/m][n/m/f][n/m/f].");
        return;
    }

    /*
        What we want here:
            * The first character is the type of element we convert;
                * 'f' = Field
                * 'c' = Class
                * 'p' = Package
                * 'm' = Method
            * The second and the third characters are the natures of the element we convert it
              from and to, respectively.
                * 'n' = Notchian
                * 'm' = MCP/Searge
                * 'f' = Forge (common name)
    */

    /* Get the first character */
    let element_type = &chars[0];

    /* Get the second and third characters */
    let from = &chars[1];
    let to = &chars[2];




}