grammar LolCode;

HAI			: '#HAI' | '#hai';
KBYE 			: '#KBYE' | '#kbye';
OBTW 			: '#OBTW' | '#obtw';
TLDR 			: '#TLDR' | '#tldr';
MAEK 			: '#MAEK' | '#maek';
HEAD 			: 'HEAD' | 'head';
PARAGRAF		: 'PARAGRAF' | 'paragraf';
LIST			: 'LIST' | 'list';
IHAZ			: '#IHAZ' | '#ihaz';
ITIZ			: '#ITIZ' | '#itiz';
MKAY			: '#MKAY' | '#mkay';
GIMMEH 			: '#GIMMEH' | '#gimmeh';
TITLE 			: 'TITLE' | 'title';
BOLD 			: 'BOLD' | 'bold';
ITALICS			: 'ITALICS' | 'italics';
ITEM 			: 'ITEM' | 'item';
LINX			: 'LINX' | 'linx';
LEMMESEE		: '#LEMMESEE' | '#lemmesee';
OIC 			: '#OIC' | '#oic';
NEWLINE			: '#NEWLINE' | '#newline';
TEXT			: CHAR+;
// Commented out VARNAME will deal with enforcing letters only semantically
// Instead of VATRNAME we will use TEXT 
// VARNAME 		: LETTER+; 
// Commented out since only used in VARNAME
//fragment LETTER		: 'A'..'Z'
//             			|  'a'..'z';
fragment CHAR     	: 'A'..'Z'
             			| 'a'..'z'
             			| '0'..'9'
             			| ',' | '.' | '"' | ':' | '?' | '!' | '%' | '/';
WS 			: (' ' | '\t' | '\r' | '\n') {skip();} ;

lolcode 		: HAI variable_define* comment* head? inner_lolcode? KBYE;
inner_lolcode		: inner_lolcode_item+;
inner_lolcode_item	: paragraph
				| bold
				| italics
				| list
				| link
				| new_line
                     		| variable_use
				| TEXT;
head			: MAEK HEAD variable_define* comment* title MKAY;
title			: GIMMEH TITLE variable_define* comment* TEXT* OIC;
comment			: OBTW TEXT* TLDR;
paragraph		: MAEK PARAGRAF variable_define* comment* inner_paragraph? MKAY;
inner_paragraph		: inner_text+;
inner_text		: TEXT
                      		| bold
                      		| italics
                      		| link
				| list
                     		| variable_use
                     		| new_line;
// REPLACE VARENAME WITH TEXT FOR BOTH variable_define AND variable_use
variable_define		: IHAZ TEXT ITIZ TEXT* MKAY;
variable_use		: LEMMESEE TEXT OIC;
bold         		: GIMMEH BOLD variable_define* bold_content? OIC;
bold_content 		: bold_item+;
bold_item   		: TEXT
                		| variable_use;
italics			: GIMMEH ITALICS variable_define* italics_content OIC;
italics_content		: italics_item+;
italics_item		: TEXT
                    		| variable_use;
list			: MAEK LIST variable_define* comment* inner_list MKAY;
list_item		: GIMMEH ITEM variable_define* comment* inner_list_item? OIC;
inner_list_item		: inner_list_item_element+;

inner_list_item_element	: TEXT
				| bold
				| italics
				| variable_use;
inner_list		: list_item+;
link 			: GIMMEH LINX variable_define* comment* link_content  OIC;
link_content 		: link_item+;
link_item 		: TEXT
				| variable_use;
new_line		: NEWLINE;
