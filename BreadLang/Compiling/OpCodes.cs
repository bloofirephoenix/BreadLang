namespace BreadLang.Compiling;

public enum OpCodes
{
    Nop  = 0b0000,
    Lw   = 0b0001,
    Sw   = 0b0010,
    Mw   = 0b0011,
    Push = 0b0100,
    Pop  = 0b0101,
    Lda  = 0b0110,
    Jmp  = 0b0111,
    Jz   = 0b1000,
    Jo   = 0b1001,
    Add  = 0b1010,
    Sub  = 0b1011,
    //     0b1100
    Tel  = 0b1101,
    Out  = 0b1110,
    Hlt  = 0b1111,
}