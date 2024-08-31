package repl

import (
	"bufio"
	"fmt"
	"io"
	"monkey/lexer"
)

const PROMPT = ">> "

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Fprint(out, PROMPT)
		scanned := scanner.Scan()
		if !scanned {
			return
		}

		line := scanner.Text()
		l := lexer.New(line)

		for tok := range l.Iterator() {
			fmt.Fprintf(out, "%+v\n", tok)
		}
	}
}
