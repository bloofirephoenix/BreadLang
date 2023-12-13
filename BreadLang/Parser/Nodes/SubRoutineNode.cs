using BreadLang.Parser.Nodes.InstructionNodes;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public class SubRoutineNode(ProgramNode programNode, string name) : Node
{
    public override void Populate(Parser parser)
    {
        parser.SkipNewLines();

        // expect indent token
        var indent = parser.Expect(TokenType.Indent);

        if (!programNode.CheckIndent(indent))
        {
            ErrorHandler.Instance!.Error(indent, "Invalid indent");
        }

        while (!parser.IsAtEnd())
        {
            var token = parser.Advance();
            Node? node = null;
            Console.WriteLine(parser.Current());
            switch (token.Type)
            {
                case TokenType.NewLine:
                    if (!programNode.CheckIndent(parser.Peek())) // todo peek next to see if its a blank line. do not exit subroutine until a non blank line fails indent check
                    {
                        Console.WriteLine("exiting subroutine");
                        return;
                    }
                    parser.Advance();
                    break;

                // instructions
                case TokenType.Nop: node = new NopNode(); break;
                case TokenType.Lw: node = new LwNode(); break;
                case TokenType.Sw: node = new SwNode(); break;
                case TokenType.Mw: node = new MwNode(); break;
                case TokenType.Push: node = new PushNode(); break;
                case TokenType.Pop: node = new PopNode(); break;
                case TokenType.Lda: node = new LdaNode(); break;
                case TokenType.Jmp: node = new JmpNode(); break;
                case TokenType.Jz: node = new JzNode(); break;
                case TokenType.Jo: node = new JoNode(); break;
                case TokenType.Add: node = new AddNode(); break;
                case TokenType.Sub: node = new SubNode(); break;
                case TokenType.Out: node = new OutNode(); break;
                case TokenType.Hlt: node = new HltNode(); break;

                case TokenType.Eof:
                    return;

                default:
                    ErrorHandler.Instance!.Error(token, $"Unexpected token {token.Type}. Expected an instruction");
                    return;
            }

            if (node == null) continue;

            node.Populate(parser);
            Children.Add(node);
        }
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return $"Subroutine({name})";
    }
}