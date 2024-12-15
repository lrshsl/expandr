" expandr syntax highlighting
" author: lrshsl

if v:version < 600
	syntax clear
elseif exists('b:current_syntax')
	finish
endif

syn keyword		exrDefine				map df
syn match		exrBecomes				"=>"

syn match		exrIdent					/[a-zA-Z][-a-zA-Z0-9]*/

syn region		exrOutString			start=/'/		end=/'/		keepend contains=exrExpr,exrString
syn region		exrOutMultiString		start=/''''/	end=/''''/	keepend contains=exrExpr,exrString
syn region		exrString				start=/"/		end=/"/		keepend contains=exrExpr contained
syn region		exrExpr					start=/\(^\|[^\\]\)\zs\[/ end=/\]/ skip=/\\\]/ keepend extend contains=ALLBUT,exrBecomes,exrDefine

syn match		exrComment				/||[^|]*\(\n\|||\)\||[^|]*\(\n\||\)/


hi def link		exrDefine				Keyword
hi def link		exrBecomes				Keyword

hi def link		exrOutString			String
hi def link		exrOutMultiString		String
hi def link		exrString				Orange

hi def link		exrExpr					Special
hi def link		exrMapName				Special

hi def link		exrComment				Comment


let b:current_syntax="exr"
" vim: noet sw=3 sts=3 ts=3
