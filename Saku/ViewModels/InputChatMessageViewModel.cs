namespace Saku.ViewModels;

public record InputChatMessageViewModel(
    ulong DiscordUserId,
    ulong DiscordChannelId,
    string Message,
    string UserName,
    DateTime CreatedAt
);