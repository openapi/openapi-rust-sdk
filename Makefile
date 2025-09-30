# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
#                                                                             #
#     ___                               _                                     #
#    / _ \ _ __   ___ _ __   __ _ _ __ (_)®                                   #
#   | | | | '_ \ / _ \ '_ \ / _` | '_ \| |                                    #
#   | |_| | |_) |  __/ | | | (_| | |_) | |                                    #
#    \___/| .__/ \___|_| |_|\__,_| .__/|_|                                    #
#         |_|                     |_|                                         #
#                                                                             #
#   The Largest Certified API Marketplace                                     #
#   Accelerate Digital Transformation • Simplify Processes • Lead Industry    #
#                                                                             #
#   ═══════════════════════════════════════════════════════════════════════   #
#                                                                             #
#   Project:        openapi-rust-sdk                                          #
#   Version:        0.1.0                                                     #
#   Author:         [Author Name]                                             #
#   Copyright:      © 2025 Openapi®. All rights reserved.                     #
#   License:        MIT                                                       #
#   Maintainer:     Francesco Bianco                                          #
#   Contact:        [Contact Information]                                     #
#   Repository:     [Repository URL]                                          #
#   Documentation:  [Docs URL]                                                #
#                                                                             #
#   ═══════════════════════════════════════════════════════════════════════   #
#                                                                             #
#   "Truth lies at the source of the stream."                                 #
#                                  — English Proverb                          #
#                                                                             #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

dev-push:
	@git add .
	@git commit -m "$$(read -p 'Commit message: ' msg; echo $$msg)" || true
	@git push
