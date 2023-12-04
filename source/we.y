%{
#include <llvm-c/Core.h>
#include <llvm-c/ExecutionEngine.h>
#include <llvm-c/Target.h>
#include <llvm-c/Analysis.h>
#include <llvm-c/BitWriter.h>

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int yylex();
int yyerror(char *s);
struct Node* root;
%}

%token INTEGER
%token IDENTIFIER
%token IDENTIFIER_LIST
%token STRING
%token LIST 
%token LIST_START
%token LIST_END
%token LIST_SEPARATOR
%token FUNCTION
%token FUNCTION_START
%token FUNCTION_END
%token CLAUSE_SEPARATOR 
%token MAP
%token MAP_START
%token MAP_END 
%token KEY_VALUE_PAIR
%token TYPE_PARAMETER_START
%token TYPE_PARAMETER_END
%token TYPE_ALIAS
%token ARRAY_TYPE
%token TUPLE_TYPE
%token TYPE_IDENTITY 
%token DEFINE_SYMBOL
%token MACRO
%token MACRO_SYMBOL
%token MACRO_LIST
%token DISCARD_SYMBOL
%token UNKNOWN 
%token TERM
%token DISCARD
%token EMPTY

%union {
  int number;
  char name[256];
  struct Node *node;
}

%type <name> INTEGER
%type <name> IDENTIFIER
%type <name> STRING

%type <node> program
%type <node> terms
%type <node> term
%type <node> statements
%type <node> statement
%type <node> typeParameter
%type <node> identifiers
%type <node> identifier
%type <node> integer
%type <node> string
%type <node> discard
%type <node> list
%type <node> listMiddle
%type <node> map
%type <node> mapMiddle
%type <node> keyValuePair
%type <node> function
%type <node> clauses
%type <node> clause
%type <node> typeAlias
%type <node> typeIdentity
%type <node> macro
%type <node> macroList

%code requires {
  struct Node {
    int type;
    char value[256];
    struct Node* child;
    struct Node* next;
  };
}

%code {
  struct Node* getEmptyNode() {
    return NULL;
  }

  // Unfortunately, it'd be good to memorize
  // "type, value, child, next"
  // This method is used a lot and we should
  // find a less brittle way to abstract the
  // construction of the object.
  struct Node* createNode(
    int type, 
    char value[256],
    struct Node* child,
    struct Node* next) 
  {
    struct Node* ptr = (struct Node*)malloc(sizeof(struct Node));

    if (ptr == NULL) 
    {
      // TODO: Emit allocation error and quit.
      return NULL;
    }
    
    ptr->type = type;

    if (value[0] != '\0') { strcpy(ptr->value, value); }
    if (child != NULL) { ptr->child = child; }
    if (next != NULL) { ptr->next = next; }

    return ptr;
  }

  struct Node* createValuelessNode(
    int type,
    struct Node* child,
    struct Node* next) 
  {
    struct Node* ptr = (struct Node*)malloc(sizeof(struct Node));

    if (ptr == NULL) 
    {
      // TODO: Emit allocation error and quit.
      return NULL;
    }
    
    ptr->type = type;

    if (child != getEmptyNode()) { ptr->child = child; }
    if (next != getEmptyNode()) { ptr->next = next; }

    return ptr;
  }
}

%%

program: terms {
  root = $$;
};

terms:
  term
  | terms term { 
    // Note that this is in reverse.
    $$ = $2;
    $$->next = $1;
  };

term: macroList DEFINE_SYMBOL identifier statement { 
  $$ = createNode(TERM, $3->value, $4, NULL);
};

statements: 
  statement
  | statements statement { 
    // Note that this is in reverse.
    $$ = $2;
    $$->next = $1;
  };

statement: 
  identifier 
  | integer 
  | macro
  | typeParameter function { $$ = $2; } /* ignoring types for now */
  | typeParameter string { $$ = $2; } /* ignoring types for now */
  | typeParameter list { $$ = $2; } /* ignoring types for now */
  | typeParameter map { $$ = $2; } /* ignoring types for now */
  | typeParameter typeAlias { $$ = $2; } /* ignoring types for now */
  | typeParameter typeIdentity { $$ = $2; } /* ignoring types for now */
  | discard;

typeParameter: 
  %empty { $$ = getEmptyNode(); } 
  | TYPE_PARAMETER_START TYPE_PARAMETER_END { $$ = getEmptyNode(); }
  | TYPE_PARAMETER_START identifiers LIST_SEPARATOR TYPE_PARAMETER_END { 
    // This defines a simple array
    $$ = createValuelessNode(ARRAY_TYPE, $2, getEmptyNode());
  }
  | TYPE_PARAMETER_START identifiers LIST_SEPARATOR integer TYPE_PARAMETER_END {
    // This defines how many elements are in the array.
    $$ = createValuelessNode(ARRAY_TYPE, $2, getEmptyNode());
  }
  | TYPE_PARAMETER_START identifiers TYPE_PARAMETER_END {
    // Multiple identifiers means it's multiple types, so a tuple.
    $$ = createValuelessNode(TUPLE_TYPE, $2, getEmptyNode());
  }; /* TODO: Other type parameter types */

identifiers:
  identifier { $$ = createNode(IDENTIFIER_LIST, $1->value, getEmptyNode(), getEmptyNode()); }
  | identifiers LIST_SEPARATOR identifier { 
    // Note that this is in reverse.
    $$ = $3;
    $$->next = $1;
  };

identifier:
  IDENTIFIER { $$ = createNode(IDENTIFIER, $1, getEmptyNode(), getEmptyNode()); };

integer:
  INTEGER { $$ = createNode(INTEGER, $1, getEmptyNode(), getEmptyNode()); };

string:
  STRING { $$ = createNode(STRING, $1, getEmptyNode(), getEmptyNode()); };

discard:
  DISCARD_SYMBOL { $$ = createValuelessNode(DISCARD, getEmptyNode(), getEmptyNode()); };

list:
  LIST_START LIST_END { $$ = createValuelessNode(LIST, getEmptyNode(), getEmptyNode()); }
  | LIST_START listMiddle LIST_END { $$ = $2; };

listMiddle:
  statement { $$ = createValuelessNode(LIST, getEmptyNode(), $1); }
  | listMiddle LIST_SEPARATOR statement { 
    // Note that this is in reverse.
    $$ = $3;
    $$->next = $1;
  };

map:
  MAP_START MAP_END { $$ = createValuelessNode(MAP, getEmptyNode(), getEmptyNode()); }
  | MAP_START mapMiddle MAP_END { 
    $$ = $2;
  };

mapMiddle:
  keyValuePair { $$ = createValuelessNode(MAP, getEmptyNode(), $1); }
  | mapMiddle LIST_SEPARATOR keyValuePair { 
    // Note that this is in reverse.
    $$ = $3;
    $$->next = $1;
  };

keyValuePair:
  identifier DEFINE_SYMBOL statement { 
    $$ = createNode(KEY_VALUE_PAIR, $1->value, $3, getEmptyNode()); 
  };

function:
  FUNCTION_START FUNCTION_END { $$ = createValuelessNode(FUNCTION, getEmptyNode(), getEmptyNode()); }
  | FUNCTION_START clauses FUNCTION_END { 
    // Note that this is in reverse.
    $$ = $2;
  };

clauses:
  clause
  | clauses CLAUSE_SEPARATOR clause { 
    // Note that this is in reverse.
    $$ = $3;
    $$->next = $1;
  };

clause:
  statement
  | statements statement { 
    // Note that this is in reverse.
    $$ = $2;
    $$->next = $1;
  };

typeAlias:
  TYPE_ALIAS statement { $$ = createValuelessNode(TYPE_ALIAS, $2, getEmptyNode()); };

typeIdentity:
  TYPE_IDENTITY statement { $$ = createValuelessNode(TYPE_IDENTITY, $2, getEmptyNode()); };

macro:
  MACRO_SYMBOL typeParameter function { $$ = createValuelessNode(MACRO, $3, getEmptyNode()); };

macroList:
  %empty { $$ = getEmptyNode(); }
  | macroList MACRO_SYMBOL identifier { 
    // Note that this is in reverse.
    $$ = $3;
    $$->next = $1;
  };

%%

extern int yylex(void);

int yyerror(char *s)
{
	printf("Syntax Error on line %s\n", s);
	return 0;
}

char* getType(int type)
{
  switch (type) 
  {
    case INTEGER:
      return "INTEGER";
      break;
    case IDENTIFIER:
      return "IDENTIFIER";
      break;
    case IDENTIFIER_LIST:
      return "IDENTIFIER_LIST";
      break;
    case STRING:
      return "STRING";
      break;
    case LIST :
      return "LIST";
      break;
    case LIST_START:
      return "LIST_START";
      break;
    case LIST_END:
      return "LIST_END";
      break;
    case LIST_SEPARATOR:
      return "LIST_SEPARATOR";
      break;
    case FUNCTION:
      return "FUNCTION";
      break;
    case FUNCTION_START:
      return "FUNCTION_START";
      break;
    case FUNCTION_END:
      return "FUNCTION_END";
      break;
    case CLAUSE_SEPARATOR :
      return "CLAUSE_SEPARATOR";
      break;
    case MAP:
      return "MAP";
      break;
    case MAP_START:
      return "MAP_START";
      break;
    case MAP_END :
      return "MAP_END";
      break;
    case KEY_VALUE_PAIR:
      return "KEY_VALUE_PAIR";
      break;
    case TYPE_PARAMETER_START:
      return "TYPE_PARAMETER_START";
      break;
    case TYPE_PARAMETER_END:
      return "TYPE_PARAMETER_END";
      break;
    case TYPE_ALIAS:
      return "TYPE_ALIAS";
      break;
    case ARRAY_TYPE:
      return "ARRAY_TYPE";
      break;
    case TUPLE_TYPE:
      return "TUPLE_TYPE";
      break;
    case TYPE_IDENTITY :
      return "TYPE_IDENTITY";
      break;
    case DEFINE_SYMBOL:
      return "DEFINE_SYMBOL";
      break;
    case MACRO:
      return "MACRO";
      break;
    case MACRO_SYMBOL:
      return "MACRO_SYMBOL";
      break;
    case MACRO_LIST:
      return "MACRO_LIST";
      break;
    case DISCARD_SYMBOL:
      return "DISCARD_SYMBOL";
      break;
    case UNKNOWN :
      return "UNKNOWN";
      break;
    case TERM:
      return "TERM";
      break;
    case DISCARD:
      return "DISCARD";
      break;
    case EMPTY:
      return "EMPTY";
      break;
  }
}

void printNodes(struct Node* root, int depth)
{
  if (root == getEmptyNode()) { return; }

  char prefix[128] = "";
  for (int i = 0; i < depth; i++)
  {
    prefix[i] = '-';
  }

  printf("%s type:%s|value:%s\n", prefix, getType(root->type), root->value);

  printNodes(root->child, depth + 1);
  printNodes(root->next, depth);
}

void toLlvm(struct Node* currentNode, LLVMModuleRef module)
{
  switch (currentNode->type) 
  {
    case INTEGER:
      printf("INTEGER");
      break;
    case IDENTIFIER:
      printf("IDENTIFIER");
      break;
    case IDENTIFIER_LIST:
      printf("IDENTIFIER_LIST");
      break;
    case STRING:
      printf("STRING");
      break;
    case LIST :
      printf("LIST");
      break;
    case LIST_START:
      printf("LIST_START");
      break;
    case LIST_END:
      printf("LIST_END");
      break;
    case LIST_SEPARATOR:
      printf("LIST_SEPARATOR");
      break;
    case FUNCTION:
      printf("FUNCTION");
      break;
    case FUNCTION_START:
      printf("FUNCTION_START");
      break;
    case FUNCTION_END:
      printf("FUNCTION_END");
      break;
    case CLAUSE_SEPARATOR :
      printf("CLAUSE_SEPARATOR");
      break;
    case MAP:
      printf("MAP");
      break;
    case MAP_START:
      printf("MAP_START");
      break;
    case MAP_END :
      printf("MAP_END");
      break;
    case KEY_VALUE_PAIR:
      printf("KEY_VALUE_PAIR");
      break;
    case TYPE_PARAMETER_START:
      printf("TYPE_PARAMETER_START");
      break;
    case TYPE_PARAMETER_END:
      printf("TYPE_PARAMETER_END");
      break;
    case TYPE_ALIAS:
      printf("TYPE_ALIAS");
      break;
    case ARRAY_TYPE:
      printf("ARRAY_TYPE");
      break;
    case TUPLE_TYPE:
      printf("TUPLE_TYPE");
      break;
    case TYPE_IDENTITY :
      printf("TYPE_IDENTITY");
      break;
    case DEFINE_SYMBOL:
      printf("DEFINE_SYMBOL");
      break;
    case MACRO:
      printf("MACRO");
      break;
    case MACRO_SYMBOL:
      printf("MACRO_SYMBOL");
      break;
    case MACRO_LIST:
      printf("MACRO_LIST");
      break;
    case DISCARD_SYMBOL:
      printf("DISCARD_SYMBOL");
      break;
    case UNKNOWN :
      printf("UNKNOWN");
      break;
    case TERM:
      printf("TERM");
      break;
    case DISCARD:
      printf("DISCARD");
      break;
    case EMPTY:
      printf("EMPTY");
      break;
  }
}

int main()
{
  yyparse();

  printNodes(root, 1);

  LLVMModuleRef mod = LLVMModuleCreateWithName("default_module");
  toLlvm(root, mod);

  return 0;
}
