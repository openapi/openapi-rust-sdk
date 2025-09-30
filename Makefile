

dev-push:
	@git add .
	@git commit -m "$$(read -p 'Commit message: ' msg; echo $$msg)" || true
	@git push
