using Saku.ViewModels;

namespace Saku.Models;

public class ChatContextModel
{
    public ChatContextModel(
        ChatType chatType,
        string message,
        bool isPresentInCurrentContext,
        DateTime createdAt,
        ulong channelId)
    {
        ChatType = chatType;
        Message = message;
        IsPresentInCurrentContext = isPresentInCurrentContext;
        CreatedAt = createdAt;
        ChannelId = channelId;
    }

    public int Id { get; set; }
    public ChatType ChatType { get; set; }
    public string Message { get; set; }
    public bool IsPresentInCurrentContext { get; set; }
    public DateTime CreatedAt { get; set; }
    public int? UserId { get; set; }
    public UserModel? User { get; set; }
    public ulong ChannelId { get; set; }
    public ChannelModel? Channel { get; set; }
}