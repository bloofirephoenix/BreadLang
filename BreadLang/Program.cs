using BreadLang;
using BreadLang.Compiling;
using BreadLang.Parser;
using BreadLang.Parser.Nodes;
using BreadLang.Tokens;

Console.WriteLine("BreadLang");

// read file
var program = @"
fib:
    MW L, A     ; L = A + B
    ADD L, B
    
    MW B, A     ; B = A
    MW A, L     ; L = A
    
    JO main     ; reset
    
    OUT A
    JMP fib     ; keep going

main:
    MW L, 0
    MW A, 0
    MW B, 1
    JMP fib
";

new ErrorHandler(program);

var tokenizer = new Tokenizer(program);
var tokens = tokenizer.ScanTokens();

var parser = new Parser(tokens);

var programNode = new ProgramNode();
programNode.Populate(parser);

Compiler compiler = new();

programNode.Compile(compiler);
void PrintParser(Node node, int tabs)
{
    for (int i = 0; i < tabs; i++)
        Console.Write("  ");
    Console.WriteLine(node);
    foreach (var child in node.Children)
    {
        PrintParser(child, tabs + 1);
    }
}