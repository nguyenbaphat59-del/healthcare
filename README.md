# Title
healthcare
# description
1. Tính năng Mint (Cấp phát Token)
Mục tiêu: Tạo ra tiền (token) từ hư vô và đưa vào tài khoản của một người dùng cụ thể.

Cơ chế: * Hàm nhận vào địa chỉ to (người nhận) và amount (số lượng).

Nó kiểm tra danh sách số dư hiện tại (BALANCES) trong bộ nhớ lưu trữ (storage).

Nếu người dùng đã có tiền, nó sẽ cộng thêm. Nếu chưa có, nó sẽ tạo mới.

Ứng dụng: Thường dành cho quản trị viên, quỹ từ thiện hoặc bảo hiểm để cấp hạn mức khám chữa bệnh cho bệnh nhân.

2. Tính năng Transfer (Chuyển tiền giữa người dùng)
Mục tiêu: Cho phép người dùng A gửi token cho người dùng B.

Cơ chế:

Bảo mật: Sử dụng from.require_auth() để đảm bảo chỉ chủ sở hữu ví mới có quyền chuyển tiền của chính họ.

Kiểm tra: Nếu số dư của người gửi (from) nhỏ hơn số tiền muốn chuyển, hợp đồng sẽ dừng lại (panic!) và báo lỗi "Not enough balance".

Trừ tiền người gửi và cộng tiền cho người nhận, sau đó cập nhật lại toàn bộ danh sách số dư.

3. Tính năng Pay for Service (Thanh toán dịch vụ y tế)
Mục tiêu: Đây là tính năng "lõi", dùng để bệnh nhân trả phí cho bệnh viện.

Cơ chế:

Tương tự như transfer nhưng được thiết kế chuyên biệt cho luồng: Bệnh nhân (user) -> Bệnh viện (hospital).

Yêu cầu xác thực quyền sở hữu từ phía user.

Có thông báo lỗi riêng (Insufficient funds) để dễ dàng phân biệt với các giao dịch chuyển tiền thông thường.

4. Tính năng Balance (Kiểm tra số dư)
Mục tiêu: Xem hiện tại một địa chỉ ví đang nắm giữ bao nhiêu token.

Cơ chế:

Đây là hàm Read-only (chỉ đọc), không làm thay đổi dữ liệu trên Blockchain.

Nó truy cập vào Map chứa số dư và trả về giá trị i128. Nếu địa chỉ đó chưa bao giờ có tiền, nó mặc định trả về 0.
# contract
https://stellar.expert/explorer/testnet/contract/CDB6MHSQOVXC4JZN2TRG6GMNGZKFX6PI6T5STBPSNDEMK2TLXK7MQFCS
<img width="1919" height="911" alt="image" src="https://github.com/user-attachments/assets/08b5c494-c6b2-49c3-aa33-affc3fb4430e" />
# features

# future scope
