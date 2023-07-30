.PHONY: all compile setup verifier clean

all: verifier.sol crypt_roller

compile: crypt_roller abi.json r1cs.out

setup: verification.key proving.key

verifier: verifier.sol

clean:
	rm -rf out

crypt_roller abi.json r1cs.out: crypt_roller.zok
	mkdir -p out
	zokrates compile \
		--input "$<" \
		--output out/crypt_roller \
		--abi-spec out/abi.json \
		--r1cs out/out.r1cs

verification.key proving.key: crypt_roller
	zokrates setup \
		--input out/crypt_roller \
		--proving-key-path out/proving.key \
		--verification-key-path out/verification.key

verifier.sol: verification.key proving.key
	zokrates export-verifier \
		--input out/verification.key \
		--output out/verifier.sol
