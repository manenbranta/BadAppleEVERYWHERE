programa
{
	inclua biblioteca Arquivos --> arq
	inclua biblioteca Tipos --> tp
	inclua biblioteca Util --> utl

	const cadeia precarlos = "../common/res/out"
	const inteiro total = 6572
	cadeia carlos = "0001"
	inteiro endereco
	funcao inicio()
	{
		escreva("                                       Bad Apple!!                           \n\n")
		utl.aguarde(1000)

		inteiro frame = 1
		enquanto(frame < total) {
			carlos = precarlos+format(frame)+".jpg.txt"
			endereco = arq.abrir_arquivo(carlos, arq.MODO_LEITURA)
			printf(frame)
			frame++
			arq.fechar_arquivo(endereco)
		}
	}

	funcao cadeia format(inteiro num) {
		se (num < 10) {retorne "000"+num}
		senao se (num < 100) {retorne "00"+num}
		senao se (num < 1000) {retorne "0"+num}
		senao {retorne tp.inteiro_para_cadeia(num,10)}
	}

	funcao printf(inteiro frame) {
		cadeia ba = ""
		
		para(inteiro linha = 1; linha < 36; linha++) {
			ba = arq.ler_linha(endereco)
			escreva (ba + "\n")
		}
		utl.aguarde(4)
		limpa()
	}
}

/* $$$ Portugol Studio $$$ 
 * 
 * Esta seção do arquivo guarda informações do Portugol Studio.
 * Você pode apagá-la se estiver utilizando outro editor.
 * 
 * @POSICAO-CURSOR = 354; 
 * @PONTOS-DE-PARADA = ;
 * @SIMBOLOS-INSPECIONADOS = ;
 * @FILTRO-ARVORE-TIPOS-DE-DADO = inteiro, real, logico, cadeia, caracter, vazio;
 * @FILTRO-ARVORE-TIPOS-DE-SIMBOLO = variavel, vetor, matriz, funcao;
 */