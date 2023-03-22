public class TypeNode extends AstNode {
    public final Lexer.TokenType type;

    public TypeNode(Lexer.TokenType type) {
        this.type = type;
    }
}
