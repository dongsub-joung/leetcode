# https://velog.io/@homil9876/Python-328.-Odd-Even-Linked-List
class Solution:
    def oddEvenList(self, head):
   
        if not head:
            return head
        
        odd = head 
        even = head.next 
        eHead = even      
        
        while even and even.next:
        
            odd.next = odd.next.next
            even.next = even.next.next
            
            odd = odd.next
            even = even.next
        
        odd.next = eHead 
        
        return head 