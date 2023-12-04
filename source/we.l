%{
#include <stdio.h>
#include <string.h>
#include "we.tab.h"
%}

integer [0-9]+
identifier ([a-zA-Z][a-zA-Z0-9_]+|`.*`)
string \"[^\n"]+\"

listStart \[
listEnd \]
mapStart \{
mapEnd \}
functionStart \(
functionEnd \)
clauseSeparator ;
listSeparator ,
typeParameterStart <
typeParameterEnd >
typeAlias '
typeIdentity \*
define :
macroSymbol @
discardSymbol _
comment #.+

%%

{integer} { sscanf(yytext, "%s", yylval.name); return (INTEGER); }
{identifier} { sscanf(yytext, "%s", yylval.name); return (IDENTIFIER); }
{string} { sscanf(yytext, "%s", yylval.name); return (STRING); }
{listStart} { return (LIST_START); }
{listEnd} { return (LIST_END); }
{mapStart} { return (MAP_START); }
{mapEnd} { return (MAP_END); }
{functionStart} { return (FUNCTION_START); }
{functionEnd} { return (FUNCTION_END); }
{clauseSeparator} { return (CLAUSE_SEPARATOR); }
{listSeparator} { return (LIST_SEPARATOR); }
{typeParameterStart} { return (TYPE_PARAMETER_START); }
{typeParameterEnd} { return (TYPE_PARAMETER_END); }
{typeAlias} { return (TYPE_ALIAS); }
{typeIdentity} { return (TYPE_IDENTITY); }
{define} { return (DEFINE_SYMBOL); }
{macroSymbol} { return (MACRO_SYMBOL); }
{discardSymbol} { return (DISCARD_SYMBOL); }
{comment} ;
[[:space:]]+ ;
. { return (UNKNOWN); }

%%
