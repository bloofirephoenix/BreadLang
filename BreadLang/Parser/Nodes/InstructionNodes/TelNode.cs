using BreadLang.Compiling;
using BreadLang.Parser.Nodes;

namespace BreadLang;

public class TelNode : Node
{
	public override void Compile(Compiler compiler)
	{
		Node node = Children[0];
		if (node is RegisterNode reg)
			compiler.WriteFirstByte(OpCodes.Tel, false, compiler.GetRegister(reg.Register));
		else if (node is NumberNode num) 
		{
			compiler.WriteFirstByte(OpCodes.Tel, true, null);
			compiler.WriteImmediate8(num.Value);
		}
		else 
		{
			throw new Exception("Expected register or number node");
		}
	}

	public override int GetSize()
	{
		throw new NotImplementedException();
	}

	public override void Populate(Parser.Parser parser)
	{
		if (parser.Check(Tokens.TokenType.Number))
			PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate8), parser);
		else if (parser.Check(Tokens.TokenType.Register))
			PopulateAndAdd(new RegisterNode(), parser);
		else
			ErrorHandler.Instance!.Error(parser.Current(), "Expected a number or register");
	}

	public override string ToString()
	{
		return "TEL";
	}
}
