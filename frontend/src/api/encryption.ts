import CryptoJS from "crypto-js";

const salt = "";
const iterations = 40000;

/**
 *
 * @param password Password used to generate key
 * @returns Generated key in hex format
 *
 */

export function generateKey(password: string): string {
  const key = CryptoJS.PBKDF2(password, salt, { keySize: 8, iterations: iterations });
  return key.toString(CryptoJS.enc.Hex);
}

/**
 *
 * @param key Key to encrypt data
 * @param plainText Data to encrypt
 * @returns ```lib.CipherParams``` format to return info about encrypted data:
 * ```js
 * var encrypted = passwordEncrypt("key", "plainText");
 *
 * var encryptedText = encrypted.toString(); //encrypted data
 *
 * ```
 */

export function passwordEncrypt(key: string, plainText: string): CryptoJS.lib.CipherParams {
  const encrypted = CryptoJS.AES.encrypt(plainText, key);
  return encrypted;
}

/**
 *
 * @param key Key to decrypt data
 * @param encryptedText Encrypted data generated by ```passwordEncrypt()```
 * @returns Decrypted data in UTF-8
 * ```js
 * var decrypted = passwordDecrypt("key", "encryptedText"); //returns decrypted data
 * ```
 */

export function passwordDecrypt(key: string, encryptedText: string): string {
  const decrypted = CryptoJS.AES.decrypt(encryptedText, key);

  return CryptoJS.enc.Utf8.stringify(decrypted);
}
