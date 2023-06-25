using Saku.Models;

namespace Saku.ViewModels;

public record ChannelRegisterViewModel(
    ulong DiscordChannelId,
    bool ChannelEnable,
    bool NoLogs
);