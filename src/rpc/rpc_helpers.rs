// boost::optional<Seed>
// parseRippleLibSeed(Json::Value const& value)
// {
//     // ripple-lib encodes seed used to generate an Ed25519 wallet in a
//     // non-standard way. While rippled never encode seeds that way, we
//     // try to detect such keys to avoid user confusion.
//     if (!value.isString())
//         return boost::none;
//
//     auto const result = decodeBase58Token(value.asString(), TokenType::None);
//
//     if (result.size() == 18 &&
//             static_cast<std::uint8_t>(result[0]) == std::uint8_t(0xE1) &&
//             static_cast<std::uint8_t>(result[1]) == std::uint8_t(0x4B))
//         return Seed(makeSlice(result.substr(2)));
//
//     return boost::none;
// }


// boost::optional<Seed>
// getSeedFromRPC(Json::Value const& params, Json::Value& error)
// {
//     // The array should be constexpr, but that makes Visual Studio unhappy.
//     static char const* const seedTypes[]
//     {
//         jss::passphrase.c_str(),
//         jss::seed.c_str(),
//         jss::seed_hex.c_str()
//     };
//
//     // Identify which seed type is in use.
//     char const* seedType = nullptr;
//     int count = 0;
//     for (auto t : seedTypes)
//     {
//         if (params.isMember (t))
//         {
//             ++count;
//             seedType = t;
//         }
//     }
//
//     if (count != 1)
//     {
//         error = RPC::make_param_error (
//             "Exactly one of the following must be specified: " +
//             std::string(jss::passphrase) + ", " +
//             std::string(jss::seed) + " or " +
//             std::string(jss::seed_hex));
//         return boost::none;
//     }
//
//     // Make sure a string is present
//     if (! params[seedType].isString())
//     {
//         error = RPC::expected_field_error (seedType, "string");
//         return boost::none;
//     }
//
//     auto const fieldContents = params[seedType].asString();
//
//     // Convert string to seed.
//     boost::optional<Seed> seed;
//
//     if (seedType == jss::seed.c_str())
//         seed = parseBase58<Seed> (fieldContents);
//     else if (seedType == jss::passphrase.c_str())
//         seed = parseGenericSeed (fieldContents);
//     else if (seedType == jss::seed_hex.c_str())
//     {
//         uint128 s;
//
//         if (s.SetHexExact (fieldContents))
//             seed.emplace (Slice(s.data(), s.size()));
//     }
//
//     if (!seed)
//         error = rpcError (rpcBAD_SEED);
//
//     return seed;
// }
