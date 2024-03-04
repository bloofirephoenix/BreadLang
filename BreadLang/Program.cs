using BreadLang;
using BreadLang.Compiling;
using BreadLang.Parser;
using BreadLang.Parser.Nodes;
using BreadLang.Tokens;

Console.WriteLine("BreadLang");

if (args.Length != 2)
{
	Console.WriteLine("Usage: BreadLang [path of program] [output path]");
	return;
}

var programPath = args[0];
var outputPath = args[1];

if (!File.Exists(programPath))
{
	Console.WriteLine($"Could not find {programPath}");
	return;
}

var program = File.ReadAllText(programPath);

if (File.Exists(outputPath))
{
	File.Delete(outputPath);
}

_ = new ErrorHandler(program);

var tokenizer = new Tokenizer(program);
var tokens = tokenizer.ScanTokens();

var parser = new Parser(tokens);

var programNode = new ProgramNode();
programNode.Populate(parser);

PrintParser(programNode, 0);

MemoryStream stream = new MemoryStream();
Compiler compiler = new(stream);

programNode.Compile(compiler);

byte[] bytes = stream.ToArray();
File.WriteAllBytes(outputPath, bytes);

foreach (var b in bytes)
{
	Console.Write($"{Convert.ToString(b, 2).PadLeft(8, '0')} ");
}

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