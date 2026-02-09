pub struct Button {
    pub pressed: bool,  // True in the first frame it is pressed
    pub held: bool,     // True while pressed
    pub released: bool, // True while released
    pub alt_pressed: bool // True if alt was pressed
}

// Ordered in the same way windows orders them
pub enum Buttons {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    NR0, NR1, NR2, NR3, NR4, NR5, NR6, NR7, NR8, NR9,
    Space,
    Enter,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Shift,
    BackSpace, //next line 
    PlusEqual,
    PeriodRightArrow,
    MinusUnderscore,
    CommaLeftArrow,
    SemiColon,
    QuestionBackSlash,
    Tilde,
    Quotes,
    Slash,
    SquareBracketsOpen,
    SquareBracketsClose,

    #[allow(non_camel_case_types)]
    BUTTONS_COUNT, // Used to determine the number of buttons
}

// static constexpr int buttonValues[BUTTONS_COUNT] =
// {
//     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
//     'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
//     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//     VK_SPACE, VK_RETURN, VK_ESCAPE, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SHIFT,
//     VK_BACK, VK_OEM_PLUS, VK_OEM_PERIOD, VK_OEM_MINUS, VK_OEM_COMMA, VK_OEM_1, VK_OEM_2, VK_OEM_3,
//     VK_OEM_7, VK_OEM_5, VK_OEM_4, VK_OEM_6,
// };

pub struct Input {
    buttons: [Button; Buttons::BUTTONS_COUNT as usize],
}


