public class BinaryOperatorNode extends ExpressionNode {
    public final Lexer.TokenType operator;
    public final ExpressionNode left;
    public final ExpressionNode right;

    public BinaryOperatorNode(Lexer.TokenType operator, ExpressionNode left, ExpressionNode right) {
        this.operator = operator;
        this.left = left;
        this.right = right;
    }
}
