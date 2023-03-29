import { ModeOfOperation, utils } from "aes-js";
import { PBKDF2, enc } from "crypto-js";

const salt = "";
const iterations = 40000;

export async function generateKey(password: string): Promise<string> {
    const key = PBKDF2(password, salt, { keySize: 8, iterations: iterations})
    return key.toString(enc.Hex);
}

export async function passwordEncrypt(key: string, iv: Buffer, plainText: string): Promise<string> {
    plainText += " ".repeat(16 - (plainText.length % 16));

    const textBytes = utils.utf8.toBytes(plainText);

    const aesCBC = new ModeOfOperation.cbc(utils.hex.toBytes(key), iv);
    const encryptedBytes = aesCBC.encrypt(textBytes);

    return utils.hex.fromBytes(encryptedBytes);
}

export async function passwordDecrypt(key: string, iv: Buffer, encryptedText: string): Promise<string> {
    const encryptedBytes = utils.hex.toBytes(encryptedText);

    const aesCBC = new ModeOfOperation.cbc(utils.hex.toBytes(key), iv);
    const decryptedBytes = aesCBC.decrypt(encryptedBytes)

    return utils.utf8.fromBytes(decryptedBytes).trimEnd();
}

