﻿using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class SwNode : Node
{
    public override void Populate(Parser parser)
    {
        var registerNode = new RegisterNode();
        registerNode.Populate(parser);
        Children.Add(registerNode);

        if (parser.Peek().Type == TokenType.Comma)
        {
            // there is an immediate value
            parser.Advance(); // comma
            var numberNode = new NumberNode(NumberNode.Type.Immediate16);
            numberNode.Populate(parser);
            Children.Add(numberNode);
        }

        parser.Advance();
    }

    public override void Compile(Compiler compiler)
    {
        RegisterSelect reg = compiler.GetRegister(((RegisterNode)Children[0]).Register);

        if (Children.Count > 1)
        {
            compiler.WriteFirstByte(OpCodes.Lw, true, reg);

            NumberNode address = (NumberNode)Children[1];
            compiler.WriteImmediate16(address.Value);
        }
        else
        {
            compiler.WriteFirstByte(OpCodes.Lw, false, reg);
        }
    }

    public override string ToString()
    {
        return "SW";
    }

    public override int GetSize()
    {
        return 1 + (Children.Count > 0 ? 2 : 0);
    }
}