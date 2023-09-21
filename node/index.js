import { Wallet, SecretNetworkClient, MsgExecuteContract} from "secretjs";
import * as fs from "fs";

const wallet = new Wallet(
  "also calm vintage aware poet green keep flock fiscal wrong ankle seek",
);

const url = "https://api.pulsar3.scrttestnet.com";

const secretjs = new SecretNetworkClient({
  url,
  chainId: "pulsar-3",
  wallet: wallet,
  walletAddress: wallet.address,
});


const lottery_contract =  "secret1ch6dauzce5hjs2umzcqykph8jh60a3vf2ntjv4";
const lottery_hash =  "f75ec61ef437552f6d01c9b6ed465b873738852d1aeeacc4e6b888426ba4ba69";
const sscrt_contract = 'secret1gvn6eap7xgsf9kydgmvpqwzkru2zj35ar2vncj';
const sscrt_hash = 'c74bc4b0406507257ed033caa922272023ab013b0c74330efc16569528fa34fe';
const viewing_key = "b7ef8f940604b155943dcebfd4b7dbcb4d34ad1c16da30de63ad01ae7c38efca";
const contract_wasm = fs.readFileSync("../contract.wasm.gz");
const codeId = 1302;


let upload_contract = async () => {
  let tx = await secretjs.tx.compute.storeCode(
    {
      sender: wallet.address,
      wasm_byte_code: contract_wasm,
      source: "",
      builder: "",
    },
    {
      gasLimit: 4_000_000,
    }
  );
	//console.log(tx);
  const codeId = Number(
    tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
      .value
  );

  console.log("codeId: ", codeId);

  const contractCodeHash = (
    await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
  ).code_hash;
  console.log(`Contract hash: ${contractCodeHash}`);
};

// upload_contract();

let instantiate_contract = async () => {
  const initMsg = {
    known_snip: sscrt_contract,
    snip_hash: sscrt_hash,
    max_bet: "100000000",
  };
  let tx = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: lottery_hash,
      init_msg: initMsg,
      label: "secret raffle" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );
  //console.log(tx);
  //Find the contract_address in the logs
  const contractAddress = tx.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  ).value;

  console.log("contract address: ", contractAddress);
};


 instantiate_contract();



 async function withdraw(){
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: lottery_contract,
    	code_hash: lottery_hash,
		msg: {
			withdraw: {
        amount: "38700000",
      },
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

// withdraw();



async function enterRaffle(){
	let hookmsg = {
    raffle: {
      quantity: 1
    }
	};
	let hookmsg64 = btoa(JSON.stringify(hookmsg));
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: sscrt_contract,
    code_hash: sscrt_hash,
		msg: {
			send: {
				recipient: lottery_contract,
        code_hash: lottery_hash,
				amount: "1000000",
				msg: hookmsg64,
			}
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

// enterRaffle();


async function endRaffle(){
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: lottery_contract,
    code_hash: lottery_hash,
		msg: {
			raffle: {},
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

// endRaffle();

async function enterBlackjack(){
	let hookmsg = {
    blackjack: {
		action: "insurance",
    }
	};
	let hookmsg64 = btoa(JSON.stringify(hookmsg));
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: sscrt_contract,
    	code_hash: sscrt_hash,
		msg: {
			send: {
				recipient: lottery_contract,
        		code_hash: lottery_hash,
				amount: "500000",
				msg: hookmsg64,
			}
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

//enterBlackjack();

async function contract(){
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: lottery_contract,
    	code_hash: lottery_hash,
		msg: {
			raffle: {},
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

//contract();

async function deposit(){
	let hookmsg = {
    deposit: {},
	}
	let hookmsg64 = btoa(JSON.stringify(hookmsg));
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: sscrt_contract,
    code_hash: sscrt_hash,
		msg: {
			send: {
				recipient: lottery_contract,
        code_hash: lottery_hash,
				amount: "100000000",
				msg: hookmsg64,
			}
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

// deposit();


async function query(){
	let stateinfo = await secretjs.query.compute.queryContract({
	  contract_address: lottery_contract,
	  code_hash: lottery_hash,
	  query: {
		  last_roulette: {
			address: secretjs.address
		},
	  }
	});
	console.log(stateinfo.last_spin.bets);
};

//query();

async function querySscrt(){
  let sscrt_info = await secretjs.query.compute.queryContract({
    contract_address: sscrt_contract,
    code_hash: sscrt_hash,
    query: {
      balance: {
        address: lottery_contract,
        key: viewing_key,
        time : Date.now()
      }
    }
  });
  console.log(sscrt_info);
};

//querySscrt();