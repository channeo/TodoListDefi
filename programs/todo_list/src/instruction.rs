
#![cfg(feature = "program")]

use crate::error::AppError;
use solana_sdk::program_error::ProgramError;
use std::{char, convert::TryInto};

#[derive(Clone, Debug, PartialEq)]
pub enum AppInstruction {
	/// 0. `[signer]` The account of the person signing transactions (wallet)
  /// 1. `[writable]` The user account to write data into
	SetUserData { name: String, premium: bool },

	/// 0. `[signer]` The account of the person signing transactions (wallet)
	/// 1. `[]` The user account
	/// 2. `[writable]` The project account to write data into
	SetProjectData { index: u32, name: String },

	/// 0. `[signer]` The account of the person signing transactions (wallet)
	/// 1. `[]` The user account
	/// 2. `[]` The project account
	/// 3. `[writable]` The task account to write data into
	SetTaskData { index: u32, message: String, completed: bool },
}

impl AppInstruction {
	pub fn unpack(instruction: &[u8]) -> Result<Self, ProgramError> {
		let (&tag, rest) = instruction
			.split_first()
			.ok_or(AppError::InvalidInstruction)?;
		Ok(match tag {
			// Set user data
			0 => {
				// Extract name from instruction
				let name_s: String = rest
					.get(..(55 * 4))
					.unwrap()
					.chunks(4)
					.map(|slice| String::from_utf8([slice[0]].to_vec()).unwrap())
					.collect();
				// Extract premium
				let premium_b = match rest.get((55 * 4)..(55 * 4 + 1)).ok_or(AppError::InvalidInstruction)? {
          [0] => false,
          [1] => true,
          _ => return Err(ProgramError::InvalidAccountData),
        };

				Self::SetUserData {
					name: name_s,
					premium: premium_b
				}
			}
			// Set project data
			1 => {
				// Extract index
				let index_u = rest
          .get(..4)
          .and_then(|slice| slice.try_into().ok())
          .map(u32::from_le_bytes)
          .ok_or(AppError::InvalidInstruction)?;
				// Extract name from instruction
				let name_s: String = rest
					.get((4)..(4 + 100 * 4))
					.unwrap()
					.chunks(4)
					.map(|slice| String::from_utf8([slice[0]].to_vec()).unwrap())
					.collect();

				Self::SetProjectData {
					index: index_u,
					name: name_s,
				}
			}
			// Set task data
			2 => {
				// Extract index
				let index_u = rest
          .get(..4)
          .and_then(|slice| slice.try_into().ok())
          .map(u32::from_le_bytes)
          .ok_or(AppError::InvalidInstruction)?;
				// Extract name from instruction
				let message_s: String = rest
					.get((4)..(4 + 140 * 4))
					.unwrap()
					.chunks(4)
					.map(|slice| String::from_utf8([slice[0]].to_vec()).unwrap())
					.collect();
				// Extract completed
				let completed_b = match rest.get((4 + 140 * 4)..(4 + 140 * 4 + 1)).ok_or(AppError::InvalidInstruction)? {
          [0] => false,
          [1] => true,
          _ => return Err(ProgramError::InvalidAccountData),
        };

				Self::SetTaskData {
					index: index_u,
					message: message_s,
					completed: completed_b
				}
			}
			_ => return Err(AppError::InvalidInstruction.into()),
		})
	}
}
