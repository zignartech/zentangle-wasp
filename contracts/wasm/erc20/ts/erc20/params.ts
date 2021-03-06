// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export class ImmutableApproveParams extends wasmtypes.ScProxy {
	amount(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ParamAmount));
	}

	delegation(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamDelegation));
	}
}

export class MutableApproveParams extends wasmtypes.ScProxy {
	amount(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ParamAmount));
	}

	delegation(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamDelegation));
	}
}

export class ImmutableInitParams extends wasmtypes.ScProxy {
	creator(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamCreator));
	}

	supply(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ParamSupply));
	}
}

export class MutableInitParams extends wasmtypes.ScProxy {
	creator(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamCreator));
	}

	supply(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ParamSupply));
	}
}

export class ImmutableTransferParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamAccount));
	}

	amount(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ParamAmount));
	}
}

export class MutableTransferParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamAccount));
	}

	amount(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ParamAmount));
	}
}

export class ImmutableTransferFromParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamAccount));
	}

	amount(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ParamAmount));
	}

	recipient(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamRecipient));
	}
}

export class MutableTransferFromParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamAccount));
	}

	amount(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ParamAmount));
	}

	recipient(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamRecipient));
	}
}

export class ImmutableAllowanceParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamAccount));
	}

	delegation(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamDelegation));
	}
}

export class MutableAllowanceParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamAccount));
	}

	delegation(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamDelegation));
	}
}

export class ImmutableBalanceOfParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamAccount));
	}
}

export class MutableBalanceOfParams extends wasmtypes.ScProxy {
	account(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamAccount));
	}
}
