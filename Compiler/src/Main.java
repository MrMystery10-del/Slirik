import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;

import java.util.LinkedList;
import java.util.List;

public class Main {
    public static void main(String[] args) throws IOException {
        File file = new File("C:\\Users\\Mr.Mystery 1.0\\Desktop\\Slirik\\Example\\src\\Main.sk");
        BufferedReader reader = new BufferedReader(new FileReader(file));

        List<Lexer.Token> tokens = new LinkedList<>();

        String s;
        while ((s = reader.readLine()) != null)
            tokens.addAll(Lexer.tokenize(s));

        Parser parser = new Parser(tokens);
        System.out.println(parser.getStatements());

        reader.close();
    }
}