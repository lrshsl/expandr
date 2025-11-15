" expandr syntax highlighting
" author: lrshsl

if v:version < 600
	syntax clear
elseif exists('b:current_syntax')
	finish
endif

syn keyword		exrDefine				map df
syn match		exrBecomes				'=>'
syn match		exrStore					'<-'
syn match		exrIsExpr				/is\|\.\.\|?/

syn match		exrNum					/\d\+/

syn match		exrExprGrp				/{\|}/
syn match		exrValGrp				/(\|)/

syn match		exrExprEnd				';'
syn match		exrNormOp				/[/]/

syn region		exrOutString			start="'"		end="'"		contains=exrExpr,exrString
syn region		exrOutMultiString		start="''''"	end="''''"	contains=exrExpr,exrString
syn region		exrString				start='"'		end='"'		contains=exrExpr
syn region		exrExpr					start="\["		end="\W"
syn match		exrExpr					"]"

syn match		exrComment				"||[^|]*\(\n\|||\)\||[^|]*\(\n\||\)"


hi	def link		ExrOp						Operator
hi def link		ExrKw						Keyword
hi def link		ExrLit					Number
hi def link		ExrGrp					Delimiter
hi def link		ExrCtrlFlow				Conditional
hi def link		ExrString				String

hi def link		exrDefine				ExrKw
hi def link		exrBecomes				ExrKw
hi def link		exrIsExpr				ExrCtrlFlow

hi def link		exrExprGrp				ExrGrp
hi def link		exrValGrp				ExrGrp

hi def link		exrStore					ExrOp
hi def link		exrExprEnd				ExrOp
hi def link		exrNormOp				ExrOp

hi def link		exrOutString			ExrString
hi def link		exrOutMultiString		ExrString
hi def link		exrString				ExrString
hi def link		exrExpr					Special

hi def link		exrComment				Comment


let b:current_syntax="exr"

" vim: noet sw=3 sts=3 ts=3
