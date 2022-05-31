const INTRO: &str = "on the fist day of xmas my true love gave to me ";

const THINGS: &'static [&'static str] = &[
    "a partridge in a pear tree",
    "2 big tacos",
    "3 things of thingy",
    "4 bag of yellow",
    "5 goooold riiings",
];

fn main() {
    let song = (1..THINGS.len()).map(|i| {
        let things_in_question = &THINGS[0..i];

        let last = things_in_question.first().unwrap();
        let rest: Vec<&str> = things_in_question[1..].iter().map(|x| *x).rev().collect();

        let these_things = if rest.len() == 0 {
            "".to_string()
        } else {
            rest.join(", ") + " and "
        };

        INTRO.to_string() + &these_things + last
    }).collect::<Vec<String>>().join(". \n");

    println!("{}", song);
}
