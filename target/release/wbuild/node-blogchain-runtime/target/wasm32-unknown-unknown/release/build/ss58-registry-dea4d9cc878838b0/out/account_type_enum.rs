# [doc = r" A known address (sub)format/network ID for SS58."] # [non_exhaustive] # [repr (u16)] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Ss58AddressFormatRegistry { # [doc = "Bare 32-bit Ed25519 public key."] BareEd25519Account = 3u16 , # [doc = "Bare 32-bit ECDSA SECP-256k1 public key."] BareSecp256K1Account = 43u16 , # [doc = "Bare 32-bit Schnorr/Ristretto (S/R 25519) public key."] BareSr25519Account = 1u16 , # [doc = "DICO - <https://dico.io>"] DicoAccount = 53u16 , # [doc = "KICO - <https://dico.io>"] KicoAccount = 52u16 , # [doc = "Acala - <https://acala.network/>"] AcalaAccount = 10u16 , # [doc = "Ajuna Network - <https://ajuna.io>"] AjunaAccount = 1337u16 , # [doc = "Altair - <https://centrifuge.io/>"] AltairAccount = 136u16 , # [doc = "Ares Protocol - <https://www.aresprotocol.com/>"] AresAccount = 34u16 , # [doc = "Astar Network - <https://astar.network>"] AstarAccount = 5u16 , # [doc = "AvN Mainnet - <https://aventus.io>"] AventusAccount = 65u16 , # [doc = "Basilisk - <https://bsx.fi>"] BasiliskAccount = 10041u16 , # [doc = "Bifrost - <https://bifrost.finance/>"] BifrostAccount = 6u16 , # [doc = "Calamari: Manta Canary Network - <https://manta.network>"] CalamariAccount = 78u16 , # [doc = "Centrifuge Chain - <https://centrifuge.io/>"] CentrifugeAccount = 36u16 , # [doc = "CESS - <https://cess.cloud>"] CessAccount = 11331u16 , # [doc = "CESS Testnet - <https://cess.cloud>"] CessTestnetAccount = 11330u16 , # [doc = "ChainX - <https://chainx.org/>"] ChainxAccount = 44u16 , # [doc = "Clover Finance - <https://clover.finance>"] CloverAccount = 128u16 , # [doc = "Composable - <https://composable.finance>"] ComposableAccount = 50u16 , # [doc = "Automata ContextFree - <https://ata.network>"] ContextfreeAccount = 11820u16 , # [doc = "CORD Network - <https://cord.network/>"] CordAccount = 29u16 , # [doc = "Crust Network - <https://crust.network>"] CrustAccount = 66u16 , # [doc = "Dark Mainnet"] DarkAccount = 17u16 , # [doc = "Darwinia Network - <https://darwinia.network/>"] DarwiniaAccount = 18u16 , # [doc = "DataHighway"] DatahighwayAccount = 33u16 , # [doc = "Dock Mainnet - <https://dock.io>"] DockMainnetAccount = 22u16 , # [doc = "Dock Testnet - <https://dock.io>"] DockTestnetAccount = 21u16 , # [doc = "Edgeware - <https://edgewa.re>"] EdgewareAccount = 7u16 , # [doc = "Efinity - <https://efinity.io/>"] EfinityAccount = 1110u16 , # [doc = "Equilibrium Network - <https://equilibrium.io>"] EquilibriumAccount = 68u16 , # [doc = "GeekCash - <https://geekcash.org>"] GeekAccount = 19u16 , # [doc = "Genshiro Network - <https://genshiro.equilibrium.io>"] GenshiroAccount = 67u16 , # [doc = "Heiko - <https://parallel.fi/>"] HeikoAccount = 110u16 , # [doc = "HydraDX - <https://hydradx.io>"] HydradxAccount = 63u16 , # [doc = "Integritee - <https://integritee.network>"] IntegriteeAccount = 13u16 , # [doc = "Integritee Incognito - <https://integritee.network>"] IntegriteeIncognitoAccount = 113u16 , # [doc = "Interlay - <https://interlay.io/>"] InterlayAccount = 2032u16 , # [doc = "Jupiter - <https://jupiter.patract.io>"] JupiterAccount = 26u16 , # [doc = "Kabocha - <https://kabocha.network>"] KabochaAccount = 27u16 , # [doc = "Kapex - <https://totemaccounting.com>"] KapexAccount = 2007u16 , # [doc = "Karura - <https://karura.network/>"] KaruraAccount = 8u16 , # [doc = "Katal Chain"] KatalchainAccount = 4u16 , # [doc = "KILT Chain - <https://kilt.io/>"] KiltAccount = 38u16 , # [doc = "Kintsugi - <https://interlay.io/>"] KintsugiAccount = 2092u16 , # [doc = "Kulupu - <https://kulupu.network/>"] KulupuAccount = 16u16 , # [doc = "Kusama Relay Chain - <https://kusama.network>"] KusamaAccount = 2u16 , # [doc = "Laminar - <http://laminar.network/>"] LaminarAccount = 11u16 , # [doc = "Litentry Network - <https://litentry.com/>"] LitentryAccount = 31u16 , # [doc = "Litmus Network - <https://litentry.com/>"] LitmusAccount = 131u16 , # [doc = "Manta network - <https://manta.network>"] MantaAccount = 77u16 , # [doc = "MathChain mainnet - <https://mathwallet.org>"] MathchainAccount = 39u16 , # [doc = "MathChain testnet - <https://mathwallet.org>"] MathchainTestnetAccount = 40u16 , # [doc = "Moonbeam - <https://moonbeam.network>"] MoonbeamAccount = 1284u16 , # [doc = "Moonriver - <https://moonbeam.network>"] MoonriverAccount = 1285u16 , # [doc = "Neatcoin Mainnet - <https://neatcoin.org>"] NeatcoinAccount = 48u16 , # [doc = "Nodle Chain - <https://nodle.io/>"] NodleAccount = 37u16 , # [doc = "OAK Network - <https://oak.tech>"] OakAccount = 51u16 , # [doc = "OriginTrail Parachain - <https://origintrail.io>"] OrigintrailParachainAccount = 101u16 , # [doc = "Parallel - <https://parallel.fi/>"] ParallelAccount = 172u16 , # [doc = "Phala Network - <https://phala.network>"] PhalaAccount = 30u16 , # [doc = "Picasso - <https://picasso.composable.finance>"] PicassoAccount = 49u16 , # [doc = "Pioneer Network by Bit.Country - <https://bit.country>"] PioneerNetworkAccount = 268u16 , # [doc = "Polimec Chain - <https://polimec.io/>"] PoliAccount = 41u16 , # [doc = "Polkadex Mainnet - <https://polkadex.trade>"] PolkadexAccount = 88u16 , # [doc = "Polkadot Relay Chain - <https://polkadot.network>"] PolkadotAccount = 0u16 , # [doc = "PolkaFoundry Network - <https://polkafoundry.com>"] PolkafoundryAccount = 99u16 , # [doc = "PolkaSmith Canary Network - <https://polkafoundry.com>"] PolkasmithAccount = 98u16 , # [doc = "Polymesh - <https://polymath.network/>"] PolymeshAccount = 12u16 , # [doc = "Pontem Network - <https://pontem.network>"] PontemNetworkAccount = 105u16 , # [doc = "QUARTZ by UNIQUE - <https://unique.network>"] QuartzMainnetAccount = 255u16 , # [doc = "This prefix is reserved."] Reserved46Account = 46u16 , # [doc = "This prefix is reserved."] Reserved47Account = 47u16 , # [doc = "Laminar Reynolds Canary - <http://laminar.network/>"] ReynoldsAccount = 9u16 , # [doc = "Robonomics - <https://robonomics.network>"] RobonomicsAccount = 32u16 , # [doc = "ShiftNrg"] ShiftAccount = 23u16 , # [doc = "Social Network - <https://social.network>"] SocialNetworkAccount = 252u16 , # [doc = "SORA Network - <https://sora.org>"] SoraAccount = 69u16 , # [doc = "SORA Kusama Parachain - <https://sora.org>"] SoraKusamaParaAccount = 420u16 , # [doc = "Stafi - <https://stafi.io>"] StafiAccount = 20u16 , # [doc = "Subsocial"] SubsocialAccount = 28u16 , # [doc = "Subspace - <https://subspace.network>"] SubspaceAccount = 6094u16 , # [doc = "Subspace testnet - <https://subspace.network>"] SubspaceTestnetAccount = 2254u16 , # [doc = "Substrate - <https://substrate.io/>"] SubstrateAccount = 42u16 , # [doc = "Synesthesia - <https://synesthesia.network/>"] SynesthesiaAccount = 15u16 , # [doc = "Totem - <https://totemaccounting.com>"] TotemAccount = 14u16 , # [doc = "UniArts Network - <https://uniarts.me>"] UniartsAccount = 45u16 , # [doc = "Valiu Liquidity Network - <https://valiu.com/>"] VlnAccount = 35u16 , # [doc = "xx network - <https://xx.network>"] XxnetworkAccount = 55u16 , # [doc = "Zeitgeist - <https://zeitgeist.pm>"] ZeitgeistAccount = 73u16 , # [doc = "ZERO - <https://zero.io>"] ZeroAccount = 24u16 , # [doc = "ZERO Alphaville - <https://zero.io>"] ZeroAlphavilleAccount = 25u16 , } # [doc = r" All non-custom address formats (Sorted by network)"] static ALL_SS58_ADDRESS_FORMATS : [Ss58AddressFormatRegistry ; 92usize] = [Ss58AddressFormatRegistry :: BareEd25519Account , Ss58AddressFormatRegistry :: BareSecp256K1Account , Ss58AddressFormatRegistry :: BareSr25519Account , Ss58AddressFormatRegistry :: DicoAccount , Ss58AddressFormatRegistry :: KicoAccount , Ss58AddressFormatRegistry :: AcalaAccount , Ss58AddressFormatRegistry :: AjunaAccount , Ss58AddressFormatRegistry :: AltairAccount , Ss58AddressFormatRegistry :: AresAccount , Ss58AddressFormatRegistry :: AstarAccount , Ss58AddressFormatRegistry :: AventusAccount , Ss58AddressFormatRegistry :: BasiliskAccount , Ss58AddressFormatRegistry :: BifrostAccount , Ss58AddressFormatRegistry :: CalamariAccount , Ss58AddressFormatRegistry :: CentrifugeAccount , Ss58AddressFormatRegistry :: CessAccount , Ss58AddressFormatRegistry :: CessTestnetAccount , Ss58AddressFormatRegistry :: ChainxAccount , Ss58AddressFormatRegistry :: CloverAccount , Ss58AddressFormatRegistry :: ComposableAccount , Ss58AddressFormatRegistry :: ContextfreeAccount , Ss58AddressFormatRegistry :: CordAccount , Ss58AddressFormatRegistry :: CrustAccount , Ss58AddressFormatRegistry :: DarkAccount , Ss58AddressFormatRegistry :: DarwiniaAccount , Ss58AddressFormatRegistry :: DatahighwayAccount , Ss58AddressFormatRegistry :: DockMainnetAccount , Ss58AddressFormatRegistry :: DockTestnetAccount , Ss58AddressFormatRegistry :: EdgewareAccount , Ss58AddressFormatRegistry :: EfinityAccount , Ss58AddressFormatRegistry :: EquilibriumAccount , Ss58AddressFormatRegistry :: GeekAccount , Ss58AddressFormatRegistry :: GenshiroAccount , Ss58AddressFormatRegistry :: HeikoAccount , Ss58AddressFormatRegistry :: HydradxAccount , Ss58AddressFormatRegistry :: IntegriteeAccount , Ss58AddressFormatRegistry :: IntegriteeIncognitoAccount , Ss58AddressFormatRegistry :: InterlayAccount , Ss58AddressFormatRegistry :: JupiterAccount , Ss58AddressFormatRegistry :: KabochaAccount , Ss58AddressFormatRegistry :: KapexAccount , Ss58AddressFormatRegistry :: KaruraAccount , Ss58AddressFormatRegistry :: KatalchainAccount , Ss58AddressFormatRegistry :: KiltAccount , Ss58AddressFormatRegistry :: KintsugiAccount , Ss58AddressFormatRegistry :: KulupuAccount , Ss58AddressFormatRegistry :: KusamaAccount , Ss58AddressFormatRegistry :: LaminarAccount , Ss58AddressFormatRegistry :: LitentryAccount , Ss58AddressFormatRegistry :: LitmusAccount , Ss58AddressFormatRegistry :: MantaAccount , Ss58AddressFormatRegistry :: MathchainAccount , Ss58AddressFormatRegistry :: MathchainTestnetAccount , Ss58AddressFormatRegistry :: MoonbeamAccount , Ss58AddressFormatRegistry :: MoonriverAccount , Ss58AddressFormatRegistry :: NeatcoinAccount , Ss58AddressFormatRegistry :: NodleAccount , Ss58AddressFormatRegistry :: OakAccount , Ss58AddressFormatRegistry :: OrigintrailParachainAccount , Ss58AddressFormatRegistry :: ParallelAccount , Ss58AddressFormatRegistry :: PhalaAccount , Ss58AddressFormatRegistry :: PicassoAccount , Ss58AddressFormatRegistry :: PioneerNetworkAccount , Ss58AddressFormatRegistry :: PoliAccount , Ss58AddressFormatRegistry :: PolkadexAccount , Ss58AddressFormatRegistry :: PolkadotAccount , Ss58AddressFormatRegistry :: PolkafoundryAccount , Ss58AddressFormatRegistry :: PolkasmithAccount , Ss58AddressFormatRegistry :: PolymeshAccount , Ss58AddressFormatRegistry :: PontemNetworkAccount , Ss58AddressFormatRegistry :: QuartzMainnetAccount , Ss58AddressFormatRegistry :: Reserved46Account , Ss58AddressFormatRegistry :: Reserved47Account , Ss58AddressFormatRegistry :: ReynoldsAccount , Ss58AddressFormatRegistry :: RobonomicsAccount , Ss58AddressFormatRegistry :: ShiftAccount , Ss58AddressFormatRegistry :: SocialNetworkAccount , Ss58AddressFormatRegistry :: SoraAccount , Ss58AddressFormatRegistry :: SoraKusamaParaAccount , Ss58AddressFormatRegistry :: StafiAccount , Ss58AddressFormatRegistry :: SubsocialAccount , Ss58AddressFormatRegistry :: SubspaceAccount , Ss58AddressFormatRegistry :: SubspaceTestnetAccount , Ss58AddressFormatRegistry :: SubstrateAccount , Ss58AddressFormatRegistry :: SynesthesiaAccount , Ss58AddressFormatRegistry :: TotemAccount , Ss58AddressFormatRegistry :: UniartsAccount , Ss58AddressFormatRegistry :: VlnAccount , Ss58AddressFormatRegistry :: XxnetworkAccount , Ss58AddressFormatRegistry :: ZeitgeistAccount , Ss58AddressFormatRegistry :: ZeroAccount , Ss58AddressFormatRegistry :: ZeroAlphavilleAccount ,] ; # [doc = r" Names of all address formats (Sorted by network)"] static ALL_SS58_ADDRESS_FORMAT_NAMES : [& str ; 92usize] = ["BareEd25519" , "BareSecp256k1" , "BareSr25519" , "DICO" , "KICO" , "acala" , "ajuna" , "altair" , "ares" , "astar" , "aventus" , "basilisk" , "bifrost" , "calamari" , "centrifuge" , "cess" , "cess-testnet" , "chainx" , "clover" , "composable" , "contextfree" , "cord" , "crust" , "dark" , "darwinia" , "datahighway" , "dock-mainnet" , "dock-testnet" , "edgeware" , "efinity" , "equilibrium" , "geek" , "genshiro" , "heiko" , "hydradx" , "integritee" , "integritee-incognito" , "interlay" , "jupiter" , "kabocha" , "kapex" , "karura" , "katalchain" , "kilt" , "kintsugi" , "kulupu" , "kusama" , "laminar" , "litentry" , "litmus" , "manta" , "mathchain" , "mathchain-testnet" , "moonbeam" , "moonriver" , "neatcoin" , "nodle" , "oak" , "origintrail-parachain" , "parallel" , "phala" , "picasso" , "pioneer_network" , "poli" , "polkadex" , "polkadot" , "polkafoundry" , "polkasmith" , "polymesh" , "pontem-network" , "quartz_mainnet" , "reserved46" , "reserved47" , "reynolds" , "robonomics" , "shift" , "social-network" , "sora" , "sora_kusama_para" , "stafi" , "subsocial" , "subspace" , "subspace_testnet" , "substrate" , "synesthesia" , "totem" , "uniarts" , "vln" , "xxnetwork" , "zeitgeist" , "zero" , "zero-alphaville" ,] ; # [doc = r" (Sorted) prefixes to index of ALL_SS58_ADDRESS_FORMATS"] static PREFIX_TO_INDEX : [(u16 , usize) ; 92usize] = [(0u16 , 65usize) , (1u16 , 2usize) , (2u16 , 46usize) , (3u16 , 0usize) , (4u16 , 42usize) , (5u16 , 9usize) , (6u16 , 12usize) , (7u16 , 28usize) , (8u16 , 41usize) , (9u16 , 73usize) , (10u16 , 5usize) , (11u16 , 47usize) , (12u16 , 68usize) , (13u16 , 35usize) , (14u16 , 85usize) , (15u16 , 84usize) , (16u16 , 45usize) , (17u16 , 23usize) , (18u16 , 24usize) , (19u16 , 31usize) , (20u16 , 79usize) , (21u16 , 27usize) , (22u16 , 26usize) , (23u16 , 75usize) , (24u16 , 90usize) , (25u16 , 91usize) , (26u16 , 38usize) , (27u16 , 39usize) , (28u16 , 80usize) , (29u16 , 21usize) , (30u16 , 60usize) , (31u16 , 48usize) , (32u16 , 74usize) , (33u16 , 25usize) , (34u16 , 8usize) , (35u16 , 87usize) , (36u16 , 14usize) , (37u16 , 56usize) , (38u16 , 43usize) , (39u16 , 51usize) , (40u16 , 52usize) , (41u16 , 63usize) , (42u16 , 83usize) , (43u16 , 1usize) , (44u16 , 17usize) , (45u16 , 86usize) , (46u16 , 71usize) , (47u16 , 72usize) , (48u16 , 55usize) , (49u16 , 61usize) , (50u16 , 19usize) , (51u16 , 57usize) , (52u16 , 4usize) , (53u16 , 3usize) , (55u16 , 88usize) , (63u16 , 34usize) , (65u16 , 10usize) , (66u16 , 22usize) , (67u16 , 32usize) , (68u16 , 30usize) , (69u16 , 77usize) , (73u16 , 89usize) , (77u16 , 50usize) , (78u16 , 13usize) , (88u16 , 64usize) , (98u16 , 67usize) , (99u16 , 66usize) , (101u16 , 58usize) , (105u16 , 69usize) , (110u16 , 33usize) , (113u16 , 36usize) , (128u16 , 18usize) , (131u16 , 49usize) , (136u16 , 7usize) , (172u16 , 59usize) , (252u16 , 76usize) , (255u16 , 70usize) , (268u16 , 62usize) , (420u16 , 78usize) , (1110u16 , 29usize) , (1284u16 , 53usize) , (1285u16 , 54usize) , (1337u16 , 6usize) , (2007u16 , 40usize) , (2032u16 , 37usize) , (2092u16 , 44usize) , (2254u16 , 82usize) , (6094u16 , 81usize) , (10041u16 , 11usize) , (11330u16 , 16usize) , (11331u16 , 15usize) , (11820u16 , 20usize) ,] ; impl Ss58AddressFormat { # [doc = r" Network/AddressType is reserved for future use."] pub fn is_reserved (& self) -> bool { self . prefix > 16384 || matches ! (self . prefix , 46u16 | 47u16) } # [doc = r" A custom format is one that is not already known."] pub fn is_custom (& self) -> bool { ! matches ! (self . prefix , 0u16 ..= 53u16 | 55u16 ..= 55u16 | 63u16 ..= 63u16 | 65u16 ..= 69u16 | 73u16 ..= 73u16 | 77u16 ..= 78u16 | 88u16 ..= 88u16 | 98u16 ..= 99u16 | 101u16 ..= 101u16 | 105u16 ..= 105u16 | 110u16 ..= 110u16 | 113u16 ..= 113u16 | 128u16 ..= 128u16 | 131u16 ..= 131u16 | 136u16 ..= 136u16 | 172u16 ..= 172u16 | 252u16 ..= 252u16 | 255u16 ..= 255u16 | 268u16 ..= 268u16 | 420u16 ..= 420u16 | 1110u16 ..= 1110u16 | 1284u16 ..= 1285u16 | 1337u16 ..= 1337u16 | 2007u16 ..= 2007u16 | 2032u16 ..= 2032u16 | 2092u16 ..= 2092u16 | 2254u16 ..= 2254u16 | 6094u16 ..= 6094u16 | 10041u16 ..= 10041u16 | 11330u16 ..= 11331u16 | 11820u16 ..= 11820u16) } }