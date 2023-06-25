namespace Saku.Models;

[Flags]
public enum ChannelState : ulong
{
    Disable = 0,
    Enable = 1,
    NoLogs = 2
}