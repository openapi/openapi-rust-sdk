#!make

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
#                                                                             #
#      ____                               _                                   #
#     / __ \____  ___  ____  ____ _____  (_) ®                                # 
#    / / / / __ \/ _ \/ __ \/ __ `/ __ \/ /                                   #
#   / /_/ / /_/ /  __/ / / / /_/ / /_/ / /                                    #
#   \____/ .___/\___/_/ /_/\__,_/ .___/_/                                     # 
#       /_/                    /_/                                            #
#                                                                             #
#   The Largest Certified API Marketplace                                     #
#   Accelerate Digital Transformation • Simplify Processes • Lead Industry    #
#                                                                             #
#   ═══════════════════════════════════════════════════════════════════════   #
#                                                                             #
#   Project:        openapi-rust-sdk                                          #
#   Version:        0.1.0                                                     #
#   Author:         Michael Cuffaro (@maiku1008)                              #
#   Copyright:      (c) 2025 Openapi®. All rights reserved.                   #
#   License:        MIT                                                       #
#   Maintainer:     Francesco Bianco                                          #
#   Contact:        https://openapi.com/                                      #
#   Repository:     [Repository URL]                                          #
#   Documentation:  [Docs URL]                                                #
#                                                                             #
#   ═══════════════════════════════════════════════════════════════════════   #
#                                                                             #
#   "Truth lies at the source of the stream."                                 #
#                                  — English Proverb                          #
#                                                                             #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #


## ====================
## Development Commands
## ====================

dev-push:
	@git add .
	@git commit -m "$$(read -p 'Commit message: ' msg; echo $$msg)" || true
	@git push

dev-version:
	@grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2

## ==================
## Packaging Commands
## ==================

publish:
	@git add .
	@git commit -m "Update release $$(make -s dev-version)" || true
	@git push
	@cargo login
	@cargo publish
