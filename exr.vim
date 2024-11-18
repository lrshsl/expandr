" expandr syntax highlighting
" author: lrshsl

if v:version < 600
	syntax clear
elseif exists('b:current_syntax')
	finish
endif

syn keyword		exrDefine				map df
syn match		exrBecomes				"=>"

syn region		exrOutString			start="'"		end="'"		contains=exrExpr,exrString
syn region		exrOutMultiString		start="''''"	end="''''"	contains=exrExpr,exrString
syn region		exrString				start='"'		end='"'		contains=exrExpr
syn region		exrExpr					start="\["		end="\W"
syn match		exrExpr					"]"

syn match		exrComment				"||[^|]*\(\n\|||\)\||[^|]*\(\n\||\)"


hi def link		exrDefine				Keyword
hi def link		exrBecomes				Keyword

hi def link		exrOutString			String
hi def link		exrOutMultiString		String
hi def link		exrString				Orange
hi def link		exrExpr					Special

hi def link		exrComment				Comment


let b:current_syntax="exr"
" vim: noet sw=3 sts=3 ts=3
