using Saku.Models;

namespace Saku.ViewModels;

public record ChannelViewModel(
    ulong DiscordChannelId,
    ChannelState State,
    string? System,
    GptModelType Model,
    int? BoundUserId);